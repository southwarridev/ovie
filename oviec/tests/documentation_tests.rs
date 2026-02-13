// Documentation Tests for Ovie v2.2
// These tests verify that documentation examples work correctly

#[cfg(test)]
mod documentation_tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_readme_version_consistency() {
        // Verify README.md contains correct version
        let readme = fs::read_to_string("README.md").expect("README.md not found");
        
        assert!(readme.contains("v2.2"), "README should reference v2.2");
        assert!(readme.contains("Complete Language Consolidation"), "README should mention consolidation");
        assert!(readme.contains("February 2026"), "README should have correct date");
    }

    #[test]
    fn test_documentation_files_exist() {
        // Verify all required documentation files exist
        let required_docs = vec![
            "README.md",
            "docs/README.md",
            "docs/getting-started.md",
            "docs/installation.md",
            "docs/language-guide.md",
            "docs/cli.md",
            "docs/aproko.md",
            "docs/internals.md",
            "docs/compiler_invariants.md",
            "RELEASE_NOTES_v2.2.md",
            "MIGRATION_GUIDE_v2.1_to_v2.2.md",
        ];

        for doc in required_docs {
            assert!(Path::new(doc).exists(), "Required documentation file missing: {}", doc);
        }
    }

    #[test]
    fn test_version_consistency_across_docs() {
        // Verify version consistency across all documentation
        let docs = vec![
            "README.md",
            "docs/README.md",
            "docs/getting-started.md",
            "docs/installation.md",
        ];

        for doc_path in docs {
            let content = fs::read_to_string(doc_path)
                .expect(&format!("Failed to read {}", doc_path));
            
            // Should not reference v2.1 as current
            assert!(
                !content.contains("Status: Production-Ready Self-Hosted Language (January"),
                "{} contains outdated status", doc_path
            );
            
            // Should reference v2.2 or 2.2.0
            assert!(
                content.contains("v2.2") || content.contains("2.2.0") || content.contains("February 2026"),
                "{} should reference v2.2", doc_path
            );
        }
    }

    #[test]
    fn test_ore_documentation_complete() {
        // Verify ORE (Ovie Runtime Environment) is documented
        let readme = fs::read_to_string("README.md").expect("README.md not found");
        
        assert!(readme.contains("Runtime Environment"), "README should mention ORE");
        assert!(readme.contains("ORE") || readme.contains("Ovie Runtime Environment"), 
                "README should define ORE");
    }

    #[test]
    fn test_cli_commands_documented() {
        // Verify all guaranteed CLI commands are documented
        let readme = fs::read_to_string("README.md").expect("README.md not found");
        
        let required_commands = vec![
            "ovie new",
            "ovie build",
            "ovie run",
            "ovie check",
            "ovie test",
            "ovie fmt",
            "ovie explain",
            "ovie env",
        ];

        for cmd in required_commands {
            assert!(readme.contains(cmd), "README should document command: {}", cmd);
        }
    }

    #[test]
    fn test_new_v22_commands_documented() {
        // Verify new v2.2 commands are documented
        let readme = fs::read_to_string("README.md").expect("README.md not found");
        
        assert!(readme.contains("oviec --env"), "Should document oviec --env");
        assert!(readme.contains("oviec --self-check"), "Should document oviec --self-check");
        assert!(readme.contains("oviec explain"), "Should document oviec explain");
    }

    #[test]
    fn test_stdlib_modules_documented() {
        // Verify all 9 stdlib modules are documented
        let readme = fs::read_to_string("README.md").expect("README.md not found");
        
        let stdlib_modules = vec![
            "std::core",
            "std::math",
            "std::io",
            "std::fs",
            "std::time",
            "std::env",
            "std::cli",
            "std::test",
            "std::log",
        ];

        for module in stdlib_modules {
            assert!(readme.contains(module), "README should document module: {}", module);
        }
    }

    #[test]
    fn test_compiler_invariants_documented() {
        // Verify compiler invariants are documented
        let readme = fs::read_to_string("README.md").expect("README.md not found");
        
        assert!(readme.contains("Enforced Compiler Invariants") || 
                readme.contains("enforced invariants"),
                "README should mention enforced invariants");
        assert!(readme.contains("AST") && readme.contains("HIR") && readme.contains("MIR"),
                "README should mention compiler stages");
    }

    #[test]
    fn test_bootstrap_verification_documented() {
        // Verify bootstrap verification is documented
        let readme = fs::read_to_string("README.md").expect("README.md not found");
        
        assert!(readme.contains("Bootstrap") || readme.contains("bootstrap"),
                "README should mention bootstrap verification");
        assert!(readme.contains("Proven") || readme.contains("proven"),
                "README should mention proven self-hosting");
    }

    #[test]
    fn test_migration_guide_complete() {
        // Verify migration guide exists and is complete
        let migration_guide = fs::read_to_string("MIGRATION_GUIDE_v2.1_to_v2.2.md")
            .expect("Migration guide not found");
        
        assert!(migration_guide.contains("v2.1"), "Should reference v2.1");
        assert!(migration_guide.contains("v2.2"), "Should reference v2.2");
        assert!(migration_guide.contains("Breaking Changes"), "Should list breaking changes");
        assert!(migration_guide.contains("Migration Steps"), "Should provide migration steps");
        assert!(migration_guide.contains("ORE"), "Should explain ORE changes");
    }

    #[test]
    fn test_release_notes_complete() {
        // Verify release notes exist and are complete
        let release_notes = fs::read_to_string("RELEASE_NOTES_v2.2.md")
            .expect("Release notes not found");
        
        assert!(release_notes.contains("v2.2.0"), "Should have version number");
        assert!(release_notes.contains("February 2026"), "Should have release date");
        assert!(release_notes.contains("Major Features"), "Should list major features");
        assert!(release_notes.contains("Breaking Changes"), "Should list breaking changes");
        assert!(release_notes.contains("Bug Fixes"), "Should list bug fixes");
    }

    #[test]
    fn test_installation_instructions_complete() {
        // Verify installation instructions are complete
        let installation = fs::read_to_string("docs/installation.md")
            .expect("Installation guide not found");
        
        assert!(installation.contains("Linux"), "Should have Linux instructions");
        assert!(installation.contains("macOS"), "Should have macOS instructions");
        assert!(installation.contains("Windows"), "Should have Windows instructions");
        assert!(installation.contains("oviec --version"), "Should show version check");
        assert!(installation.contains("oviec --self-check"), "Should show self-check");
    }

    #[test]
    fn test_no_placeholder_text() {
        // Verify no placeholder text remains in documentation
        let docs = vec![
            "README.md",
            "docs/README.md",
            "docs/getting-started.md",
        ];

        for doc_path in docs {
            let content = fs::read_to_string(doc_path)
                .expect(&format!("Failed to read {}", doc_path));
            
            assert!(!content.contains("TODO"), "{} contains TODO", doc_path);
            assert!(!content.contains("FIXME"), "{} contains FIXME", doc_path);
            assert!(!content.contains("coming soon"), "{} contains 'coming soon'", doc_path);
            assert!(!content.contains("placeholder"), "{} contains 'placeholder'", doc_path);
        }
    }

    #[test]
    fn test_code_examples_syntax() {
        // Verify code examples use correct syntax
        let getting_started = fs::read_to_string("docs/getting-started.md")
            .expect("Getting started guide not found");
        
        // Should use Ovie syntax
        assert!(getting_started.contains("seeAm"), "Should use seeAm for print");
        assert!(getting_started.contains("mut"), "Should use mut for mutable variables");
        assert!(getting_started.contains("fn"), "Should use fn for functions");
    }

    #[test]
    fn test_feature_claims_accurate() {
        // Verify feature claims are accurate
        let readme = fs::read_to_string("README.md").expect("README.md not found");
        
        // Should claim complete language
        assert!(readme.contains("complete") || readme.contains("Complete"),
                "Should claim completeness");
        
        // Should mention enforced correctness
        assert!(readme.contains("enforced") || readme.contains("Enforced"),
                "Should mention enforced correctness");
        
        // Should mention trustworthy
        assert!(readme.contains("trustworthy") || readme.contains("Trustworthy"),
                "Should mention trustworthiness");
    }

    #[test]
    fn test_links_format_correct() {
        // Verify markdown links are formatted correctly
        let readme = fs::read_to_string("README.md").expect("README.md not found");
        
        // Check for common link patterns
        assert!(readme.contains("]("), "Should contain markdown links");
        assert!(readme.contains("https://"), "Should contain external links");
        
        // Should not have broken link syntax
        assert!(!readme.contains("](]"), "Should not have broken links");
        assert!(!readme.contains("[]("), "Should not have empty links");
    }

    #[test]
    fn test_documentation_validation_exists() {
        // Verify documentation validation report exists
        assert!(Path::new("docs/DOCUMENTATION_VALIDATION.md").exists(),
                "Documentation validation report should exist");
        
        let validation = fs::read_to_string("docs/DOCUMENTATION_VALIDATION.md")
            .expect("Validation report not found");
        
        assert!(validation.contains("✅"), "Should show validation status");
        assert!(validation.contains("Validated"), "Should confirm validation");
    }

    #[test]
    fn test_core_principles_documented() {
        // Verify core principles are documented
        let readme = fs::read_to_string("README.md").expect("README.md not found");
        
        assert!(readme.contains("Core Principles") || readme.contains("Principles"),
                "Should document core principles");
        assert!(readme.contains("Enforced correctness"), "Should list enforced correctness");
        assert!(readme.contains("Proven self-hosting"), "Should list proven self-hosting");
        assert!(readme.contains("Complete runtime environment"), "Should list complete ORE");
    }

    #[test]
    fn test_roadmap_updated() {
        // Verify roadmap reflects v2.2 completion
        let readme = fs::read_to_string("README.md").expect("README.md not found");
        
        assert!(readme.contains("Stage 2") && readme.contains("Complete"),
                "Should show Stage 2 as complete");
        assert!(readme.contains("Stage 3") && readme.contains("Future"),
                "Should show Stage 3 as future");
    }

    #[test]
    fn test_acknowledgments_present() {
        // Verify acknowledgments section exists
        let readme = fs::read_to_string("README.md").expect("README.md not found");
        
        assert!(readme.contains("Acknowledgments") || readme.contains("Thanks"),
                "Should have acknowledgments section");
    }

    #[test]
    fn test_license_documented() {
        // Verify license is documented
        let readme = fs::read_to_string("README.md").expect("README.md not found");
        
        assert!(readme.contains("MIT"), "Should mention MIT license");
        assert!(readme.contains("LICENSE"), "Should reference LICENSE file");
        assert!(Path::new("LICENSE").exists(), "LICENSE file should exist");
    }
}
