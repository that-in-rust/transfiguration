-- Migration Projects Research Database Schema
-- PostgreSQL schema for IDE migration project analysis

-- Main projects table for IDE migration projects
CREATE TABLE IF NOT EXISTS migration_projects (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    url TEXT NOT NULL,
    github_url TEXT,
    category VARCHAR(50) NOT NULL CHECK (category IN ('ide_migration', 'vscode_fork', 'rust_ide', 'ai_ide', 'wasm_app')),
    status VARCHAR(20) NOT NULL CHECK (status IN ('active', 'archived', 'deprecated')),
    last_updated DATE,
    stars INTEGER DEFAULT 0,
    contributors INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Technical details for migration projects
CREATE TABLE IF NOT EXISTS migration_technical_details (
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES migration_projects(id) ON DELETE CASCADE,
    source_technology VARCHAR(50) NOT NULL CHECK (source_technology IN ('electron', 'native', 'web', 'java', 'dotnet')),
    target_technology VARCHAR(50) NOT NULL CHECK (target_technology IN ('rust', 'cpp', 'go', 'wasm', 'native', 'tauri', 'flutter')),
    migration_approach VARCHAR(50) NOT NULL CHECK (migration_approach IN ('gradual', 'complete_rewrite', 'hybrid', 'incremental')),
    architecture_patterns JSONB DEFAULT '[]',
    performance_metrics JSONB DEFAULT '{}',
    migration_timeline_months INTEGER,
    team_size INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Performance metrics tracking
CREATE TABLE IF NOT EXISTS performance_metrics (
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES migration_projects(id) ON DELETE CASCADE,
    metric_type VARCHAR(50) NOT NULL CHECK (metric_type IN ('startup_time', 'memory_usage', 'cpu_usage', 'bundle_size', 'response_time')),
    before_value DECIMAL,
    after_value DECIMAL,
    unit VARCHAR(20) NOT NULL,
    improvement_percentage DECIMAL,
    measurement_context TEXT,
    source_url TEXT,
    verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Migration strategies and approaches
CREATE TABLE IF NOT EXISTS migration_strategies (
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES migration_projects(id) ON DELETE CASCADE,
    strategy_name VARCHAR(100) NOT NULL,
    description TEXT,
    phases JSONB DEFAULT '[]',
    challenges JSONB DEFAULT '[]',
    solutions JSONB DEFAULT '[]',
    timeline_estimate VARCHAR(50),
    resource_requirements JSONB DEFAULT '{}',
    success_factors JSONB DEFAULT '[]',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Technical challenges and solutions
CREATE TABLE IF NOT EXISTS technical_challenges (
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES migration_projects(id) ON DELETE CASCADE,
    challenge_category VARCHAR(50) NOT NULL,
    challenge_description TEXT NOT NULL,
    solution_approach TEXT,
    implementation_details TEXT,
    outcome VARCHAR(50) CHECK (outcome IN ('successful', 'partial', 'failed', 'ongoing')),
    lessons_learned TEXT,
    source_url TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Analysis and insights
CREATE TABLE IF NOT EXISTS project_analysis (
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES migration_projects(id) ON DELETE CASCADE,
    strengths JSONB DEFAULT '[]',
    weaknesses JSONB DEFAULT '[]',
    lessons_learned JSONB DEFAULT '[]',
    applicability_to_kiro VARCHAR(20) CHECK (applicability_to_kiro IN ('high', 'medium', 'low')),
    confidence_level VARCHAR(20) CHECK (confidence_level IN ('high', 'medium', 'low')),
    technical_score INTEGER CHECK (technical_score >= 1 AND technical_score <= 10),
    adoption_score INTEGER CHECK (adoption_score >= 1 AND adoption_score <= 10),
    sustainability_score INTEGER CHECK (sustainability_score >= 1 AND sustainability_score <= 10),
    relevance_score INTEGER CHECK (relevance_score >= 1 AND relevance_score <= 10),
    overall_score DECIMAL,
    analysis_notes TEXT,
    analyst_name VARCHAR(100),
    analysis_date DATE DEFAULT CURRENT_DATE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Source verification and attribution
CREATE TABLE IF NOT EXISTS research_sources (
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES migration_projects(id) ON DELETE CASCADE,
    source_type VARCHAR(50) NOT NULL CHECK (source_type IN ('github', 'documentation', 'blog_post', 'conference_talk', 'paper', 'interview', 'forum_post')),
    source_url TEXT NOT NULL,
    source_title TEXT,
    author VARCHAR(255),
    publication_date DATE,
    reliability_score INTEGER CHECK (reliability_score >= 1 AND reliability_score <= 5),
    content_summary TEXT,
    key_insights JSONB DEFAULT '[]',
    verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_migration_projects_category ON migration_projects(category);
CREATE INDEX IF NOT EXISTS idx_migration_projects_status ON migration_projects(status);
CREATE INDEX IF NOT EXISTS idx_technical_details_source_tech ON migration_technical_details(source_technology);
CREATE INDEX IF NOT EXISTS idx_technical_details_target_tech ON migration_technical_details(target_technology);
CREATE INDEX IF NOT EXISTS idx_performance_metrics_type ON performance_metrics(metric_type);
CREATE INDEX IF NOT EXISTS idx_project_analysis_applicability ON project_analysis(applicability_to_kiro);
CREATE INDEX IF NOT EXISTS idx_research_sources_type ON research_sources(source_type);

-- Full-text search indexes
CREATE INDEX IF NOT EXISTS idx_projects_name_search ON migration_projects USING gin(to_tsvector('english', name));
CREATE INDEX IF NOT EXISTS idx_challenges_description_search ON technical_challenges USING gin(to_tsvector('english', challenge_description));
CREATE INDEX IF NOT EXISTS idx_sources_content_search ON research_sources USING gin(to_tsvector('english', content_summary));