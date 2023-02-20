use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::e206_reverse_linked_list::{reverse_list, ListNode};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("reverse_linked_list", |b| {
        b.iter(|| {
            let root = Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode {
                                    val: 6,
                                    next: Some(Box::new(ListNode {
                                        val: 7,
                                        next: Some(Box::new(ListNode {
                                            val: 8,
                                            next: Some(Box::new(ListNode {
                                                val: 9,
                                                next: Some(Box::new(ListNode {
                                                    val: 10,
                                                    next: None,
                                                })),
                                            })),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            }));

            let output = reverse_list(black_box(root));
            let expected = Some(Box::new(ListNode {
                val: 10,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 8,
                        next: Some(Box::new(ListNode {
                            val: 7,
                            next: Some(Box::new(ListNode {
                                val: 6,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode {
                                        val: 4,
                                        next: Some(Box::new(ListNode {
                                            val: 3,
                                            next: Some(Box::new(ListNode {
                                                val: 2,
                                                next: Some(Box::new(ListNode {
                                                    val: 1,
                                                    next: None,
                                                })),
                                            })),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            }));
            assert_eq!(expected, output);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
