# Dobby Subagent Code Summarizer - Development Backlog

## 🎯 Enhanced CLI Help Improvements (Planned)

### 1. **Default Help Display**
**Problem**: Users currently see validation errors when running CLI without arguments
**Solution**: Show helpful help automatically instead of errors

**Implementation**:
- Modify `validate_args()` to detect missing required arguments
- Show help display with examples before error messages
- Guide users through required parameters with templates

### 2. **Use Case Examples in CLI Help**
**Problem**: Users need proven parameter combinations for different scenarios
**Solution**: Add copy-paste ready examples for common use cases

**Examples to Add**:
```bash
# Quick Code Review (small files < 100 lines)
cargo run --release --bin parallel_summarizer -- \
    --file ./src/utils.rs \
    --output-file /tmp/code_review.md \
    --results-file /tmp/review_progress.log \
    --loc 30 --agent-count 2 --sampling-strategy beam \
    --temperature 0.3 --max-new-tokens 50 \
    --prompt "Summarize functionality:"

# Security Analysis (medium files 100-1000 lines)
cargo run --release --bin parallel_summarizer -- \
    --file ./src/auth.rs \
    --output-file /tmp/security_analysis.md \
    --results-file /tmp/security_progress.log \
    --loc 500 --agent-count 8 --sampling-strategy beam \
    --temperature 0.4 --max-new-tokens 100 \
    --prompt "Analyze security patterns:"

# Documentation Generation (large files > 1000 lines)
cargo run --release --bin parallel_summarizer -- \
    --file ./src/lib.rs \
    --output-file /tmp/api_docs.md \
    --results-file /tmp/docs_progress.log \
    --loc 1000 --agent-count 15 --sampling-strategy beam \
    --temperature 0.5 --max-new-tokens 120 \
    --prompt "Generate API documentation:"
```

### 3. **Interactive Terminal Guidance**
**Problem**: Users need guidance when they make mistakes
**Solution**: Smart validation messages with educational content

**Implementation Approach**:
- Replace generic error messages with helpful guidance
- Include copy-paste ready command templates
- Add file size recommendations
- Provide troubleshooting tips

**Example Enhanced Error Message**:
```
❌ Missing required arguments. Here's how to get started:

🚀 Quick Start Examples:

# For small files (< 100 lines):
[copy-paste command template]

💡 Tips:
- Use --loc based on file size (30 for small, 500 for medium, 1000 for large)
- Beam search (--sampling-strategy beam) works best for consistent results
- Temperature 0.3-0.5 provides focused, reliable summaries
```

### 4. **File Size Recommendations**
**Problem**: Users don't know optimal chunk sizes for different file types
**Solution**: Add recommendations based on testing

**Recommendations**:
- **Small files** (< 100 lines): --loc 30, --agent-count 2
- **Medium files** (100-1000 lines): --loc 500, --agent-count 8
- **Large files** (> 1000 lines): --loc 1000, --agent-count 15

## 🔧 Technical Implementation Notes

### Code Changes Required:
1. **Modify `src/bin/parallel_summarizer.rs`**:
   - Update `validate_args()` function
   - Add help examples to CLI definition
   - Implement smart error messages

2. **Update CLI Help Text**:
   - Use clap's `about` and `long_about` attributes
   - Add examples section
   - Include parameter recommendations

3. **Test New Help System**:
   - Verify help displays correctly
   - Test examples are copy-paste ready
   - Ensure error messages are helpful

### Files to Modify:
- `src/bin/parallel_summarizer.rs` - Main CLI logic
- `README.md` - Update with new help features

## 📋 Priority & Timeline

**High Priority** (Next Sprint):
- Default help display
- Use case examples in help text
- Smart validation messages

**Medium Priority** (Future):
- Interactive guidance enhancements
- Advanced parameter recommendations
- Preset configurations

**Low Priority** (Nice to Have):
- Auto-detection of optimal parameters
- Learning system from user behavior
- Context-aware help based on file type

## 🧪 Testing Requirements

### Manual Testing:
- Test CLI help displays correctly
- Verify examples are accurate and working
- Check error messages are helpful

### Automated Testing:
- Add tests for help display logic
- Validate parameter recommendations
- Test error handling improvements

---

**Documented**: 2025-10-25
**Status**: Planned for next development cycle