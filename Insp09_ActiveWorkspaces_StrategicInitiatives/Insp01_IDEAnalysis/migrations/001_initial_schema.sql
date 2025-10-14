-- Create custom types
CREATE TYPE project_category AS ENUM (
    'ide_migration',
    'vscode_fork', 
    'rust_ide',
    'ai_ide',
    'wasm_app',
    'technical_pattern'
);

CREATE TYPE project_status AS ENUM (
    'discovered',
    'analyzing', 
    'analyzed',
    'verified',
    'archived'
);

CREATE TYPE confidence_level AS ENUM (
    'high',
    'medium',
    'low'
);

-- Projects table - main entity for research subjects
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    url TEXT NOT NULL UNIQUE,
    category project_category NOT NULL,
    status project_status NOT NULL DEFAULT 'discovered',
    description TEXT,
    stars INTEGER,
    contributors INTEGER,
    last_updated TIMESTAMPTZ,
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Technical analysis table
CREATE TABLE technical_analyses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    source_technology TEXT,
    target_technology TEXT,
    migration_approach TEXT,
    architecture_patterns TEXT[] NOT NULL DEFAULT '{}',
    performance_metrics JSONB NOT NULL DEFAULT '{}',
    strengths TEXT[] NOT NULL DEFAULT '{}',
    weaknesses TEXT[] NOT NULL DEFAULT '{}',
    lessons_learned TEXT[] NOT NULL DEFAULT '{}',
    applicability_to_kiro confidence_level NOT NULL DEFAULT 'medium',
    confidence_level confidence_level NOT NULL DEFAULT 'medium',
    analysis_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Research sources for attribution and verification
CREATE TABLE research_sources (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID REFERENCES projects(id) ON DELETE SET NULL,
    url TEXT NOT NULL,
    source_type TEXT NOT NULL,
    title TEXT,
    author TEXT,
    published_date TIMESTAMPTZ,
    reliability_score REAL CHECK (reliability_score >= 0 AND reliability_score <= 1),
    verification_status TEXT NOT NULL DEFAULT 'pending',
    content_hash TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Research findings and insights
CREATE TABLE research_findings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID REFERENCES projects(id) ON DELETE SET NULL,
    category TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    confidence_level confidence_level NOT NULL DEFAULT 'medium',
    tags TEXT[] NOT NULL DEFAULT '{}',
    source_ids UUID[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Competitive analysis data
CREATE TABLE competitive_analyses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    ai_integration JSONB NOT NULL DEFAULT '{}',
    extension_system JSONB NOT NULL DEFAULT '{}',
    business_model JSONB NOT NULL DEFAULT '{}',
    user_experience JSONB NOT NULL DEFAULT '{}',
    performance_data JSONB NOT NULL DEFAULT '{}',
    market_position TEXT,
    differentiation_factors TEXT[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Research progress tracking
CREATE TABLE research_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    category project_category NOT NULL UNIQUE,
    target_count INTEGER NOT NULL,
    current_count INTEGER NOT NULL DEFAULT 0,
    completion_percentage REAL NOT NULL DEFAULT 0 CHECK (completion_percentage >= 0 AND completion_percentage <= 100),
    quality_score REAL CHECK (quality_score >= 0 AND quality_score <= 1),
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    notes TEXT
);

-- Data validation results
CREATE TABLE validation_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_type TEXT NOT NULL,
    entity_id UUID NOT NULL,
    validation_type TEXT NOT NULL,
    status TEXT NOT NULL,
    score REAL CHECK (score >= 0 AND score <= 1),
    issues TEXT[] NOT NULL DEFAULT '{}',
    recommendations TEXT[] NOT NULL DEFAULT '{}',
    validated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    validator TEXT NOT NULL
);

-- Indexes for performance
CREATE INDEX idx_projects_category ON projects(category);
CREATE INDEX idx_projects_status ON projects(status);
CREATE INDEX idx_projects_name_search ON projects USING gin(to_tsvector('english', name || ' ' || COALESCE(description, '')));
CREATE INDEX idx_projects_metadata ON projects USING gin(metadata);

CREATE INDEX idx_technical_analyses_project_id ON technical_analyses(project_id);
CREATE INDEX idx_technical_analyses_confidence ON technical_analyses(confidence_level);

CREATE INDEX idx_research_sources_project_id ON research_sources(project_id);
CREATE INDEX idx_research_sources_type ON research_sources(source_type);
CREATE INDEX idx_research_sources_verification ON research_sources(verification_status);

CREATE INDEX idx_research_findings_project_id ON research_findings(project_id);
CREATE INDEX idx_research_findings_category ON research_findings(category);
CREATE INDEX idx_research_findings_tags ON research_findings USING gin(tags);
CREATE INDEX idx_research_findings_search ON research_findings USING gin(to_tsvector('english', title || ' ' || description));

CREATE INDEX idx_competitive_analyses_project_id ON competitive_analyses(project_id);

CREATE INDEX idx_validation_results_entity ON validation_results(entity_type, entity_id);
CREATE INDEX idx_validation_results_type ON validation_results(validation_type);

-- Update triggers for updated_at columns
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_projects_updated_at BEFORE UPDATE ON projects FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_technical_analyses_updated_at BEFORE UPDATE ON technical_analyses FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_research_sources_updated_at BEFORE UPDATE ON research_sources FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_research_findings_updated_at BEFORE UPDATE ON research_findings FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_competitive_analyses_updated_at BEFORE UPDATE ON competitive_analyses FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Initialize research progress tracking
INSERT INTO research_progress (category, target_count) VALUES
    ('ide_migration', 10),
    ('vscode_fork', 15),
    ('rust_ide', 8),
    ('ai_ide', 12),
    ('wasm_app', 6),
    ('technical_pattern', 20);