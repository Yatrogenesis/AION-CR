use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aion_normative::{NormativeEngine, InMemoryNormativeRepository, ComprehensiveValidator};
use aion_compliance::ComplianceFrameworkLibrary;
use std::sync::Arc;

fn benchmark_framework_loading(c: &mut Criterion) {
    c.bench_function("load_standard_frameworks", |b| {
        b.iter(|| {
            let frameworks = ComplianceFrameworkLibrary::get_all_standard_frameworks();
            black_box(frameworks)
        });
    });
}

fn benchmark_conflict_detection(c: &mut Criterion) {
    let repository = Arc::new(InMemoryNormativeRepository::new());
    let validator = Arc::new(ComprehensiveValidator::new());
    let mut engine = NormativeEngine::new(repository, validator);

    let frameworks = ComplianceFrameworkLibrary::get_all_standard_frameworks().unwrap();
    for framework in &frameworks {
        engine.register_framework(framework.clone()).unwrap();
    }

    c.bench_function("detect_conflicts", |b| {
        b.iter(|| {
            let conflicts = aion_conflict::AdvancedConflictDetector::new()
                .detect_conflicts(black_box(&frameworks));
            black_box(conflicts)
        });
    });
}

criterion_group!(benches, benchmark_framework_loading, benchmark_conflict_detection);
criterion_main!(benches);