-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create ui_schemas table
CREATE TABLE public.ui_schemas (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    prompt TEXT NOT NULL,
    schema_data JSONB NOT NULL,
    is_public BOOLEAN DEFAULT false,
    tags TEXT[] DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc'::text, now()) NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc'::text, now()) NOT NULL
);

-- Create analytics_events table for tracking usage
CREATE TABLE public.analytics_events (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    event_type TEXT NOT NULL,
    user_id UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    schema_id UUID REFERENCES public.ui_schemas(id) ON DELETE SET NULL,
    prompt TEXT,
    metadata JSONB DEFAULT '{}',
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc'::text, now()) NOT NULL
);

-- Create user_profiles table for additional user data
CREATE TABLE public.user_profiles (
    id UUID REFERENCES auth.users(id) ON DELETE CASCADE PRIMARY KEY,
    display_name TEXT,
    avatar_url TEXT,
    bio TEXT,
    website TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc'::text, now()) NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc'::text, now()) NOT NULL
);

-- Create storage buckets
INSERT INTO storage.buckets (id, name, public) VALUES ('ui-previews', 'ui-previews', true);
INSERT INTO storage.buckets (id, name, public) VALUES ('avatars', 'avatars', true);

-- Row Level Security (RLS) Policies

-- Enable RLS on all tables
ALTER TABLE public.ui_schemas ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.analytics_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.user_profiles ENABLE ROW LEVEL SECURITY;

-- UI Schemas policies
CREATE POLICY "Users can view public schemas or their own" ON public.ui_schemas
    FOR SELECT USING (is_public = true OR auth.uid() = user_id);

CREATE POLICY "Users can insert their own schemas" ON public.ui_schemas
    FOR INSERT WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can update their own schemas" ON public.ui_schemas
    FOR UPDATE USING (auth.uid() = user_id);

CREATE POLICY "Users can delete their own schemas" ON public.ui_schemas
    FOR DELETE USING (auth.uid() = user_id);

-- Analytics events policies
CREATE POLICY "Users can view their own analytics" ON public.analytics_events
    FOR SELECT USING (auth.uid() = user_id);

CREATE POLICY "Anyone can insert analytics events" ON public.analytics_events
    FOR INSERT WITH CHECK (true);

-- User profiles policies
CREATE POLICY "Users can view any profile" ON public.user_profiles
    FOR SELECT USING (true);

CREATE POLICY "Users can insert their own profile" ON public.user_profiles
    FOR INSERT WITH CHECK (auth.uid() = id);

CREATE POLICY "Users can update their own profile" ON public.user_profiles
    FOR UPDATE USING (auth.uid() = id);

-- Storage policies
CREATE POLICY "Anyone can view ui previews" ON storage.objects
    FOR SELECT USING (bucket_id = 'ui-previews');

CREATE POLICY "Authenticated users can upload ui previews" ON storage.objects
    FOR INSERT WITH CHECK (bucket_id = 'ui-previews' AND auth.role() = 'authenticated');

CREATE POLICY "Anyone can view avatars" ON storage.objects
    FOR SELECT USING (bucket_id = 'avatars');

CREATE POLICY "Users can upload their own avatars" ON storage.objects
    FOR INSERT WITH CHECK (bucket_id = 'avatars' AND auth.role() = 'authenticated');

CREATE POLICY "Users can update their own avatars" ON storage.objects
    FOR UPDATE USING (bucket_id = 'avatars' AND auth.uid()::text = (storage.foldername(name))[1]);

-- Create indexes for better performance
CREATE INDEX idx_ui_schemas_user_id ON public.ui_schemas(user_id);
CREATE INDEX idx_ui_schemas_is_public ON public.ui_schemas(is_public);
CREATE INDEX idx_ui_schemas_created_at ON public.ui_schemas(created_at DESC);
CREATE INDEX idx_ui_schemas_tags ON public.ui_schemas USING GIN(tags);
CREATE INDEX idx_analytics_events_user_id ON public.analytics_events(user_id);
CREATE INDEX idx_analytics_events_schema_id ON public.analytics_events(schema_id);
CREATE INDEX idx_analytics_events_timestamp ON public.analytics_events(timestamp DESC);

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION public.update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = timezone('utc'::text, now());
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for updated_at
CREATE TRIGGER update_ui_schemas_updated_at BEFORE UPDATE ON public.ui_schemas
    FOR EACH ROW EXECUTE FUNCTION public.update_updated_at_column();

CREATE TRIGGER update_user_profiles_updated_at BEFORE UPDATE ON public.user_profiles
    FOR EACH ROW EXECUTE FUNCTION public.update_updated_at_column();

-- Function to create user profile on signup
CREATE OR REPLACE FUNCTION public.handle_new_user()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO public.user_profiles (id, display_name)
    VALUES (NEW.id, NEW.raw_user_meta_data->>'display_name');
    RETURN NEW;
END;
$$ language 'plpgsql' security definer;

-- Trigger to create profile on signup
CREATE TRIGGER on_auth_user_created
    AFTER INSERT ON auth.users
    FOR EACH ROW EXECUTE FUNCTION public.handle_new_user();

-- Create view for schema statistics
CREATE VIEW public.schema_stats AS
SELECT
    s.id,
    s.title,
    s.user_id,
    s.created_at,
    s.is_public,
    COUNT(a.id) as view_count,
    MAX(a.timestamp) as last_viewed
FROM public.ui_schemas s
LEFT JOIN public.analytics_events a ON s.id = a.schema_id AND a.event_type = 'schema_viewed'
GROUP BY s.id, s.title, s.user_id, s.created_at, s.is_public;
