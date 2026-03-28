use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rustenium::browsers::{
    BidiBrowser, BrowserScreenshotOptionsBuilder, ChromeBrowser, ChromeConfig,
    NavigateOptionsBuilder, create_chrome_browser,
};
use rustenium::input::{
    CurveParams, MouseClickOptions, MouseMoveOptions, Point, generate_durations,
    generate_trajectory, random_curve_params,
};
use rustenium::nodes::Node;
use rustenium_bidi_definitions::browsing_context::types::ReadinessState;
use rustenium_bidi_definitions::script::types::{PrimitiveProtocolValue, RemoteValue};
use rustenium_macros::css;
use tokio::runtime::Runtime;

fn launch_headless(rt: &Runtime) -> ChromeBrowser {
    rt.block_on(async {
        let mut config = ChromeConfig::default();
        config.remote_debugging_port = Some(0);
        config.browser_flags = Some(vec![
            "--headless=new".to_string(),
            "--window-size=1280,720".to_string(),
        ]);
        create_chrome_browser(Some(config)).await
    })
}

fn nav_complete(browser: &mut ChromeBrowser, url: &str, rt: &Runtime) {
    rt.block_on(async {
        browser.navigate_with_options(
            url,
            NavigateOptionsBuilder::default()
                .wait(ReadinessState::Complete)
                .build(),
        ).await.unwrap()
    });
}

fn teardown(browser: ChromeBrowser, rt: &Runtime) {
    rt.block_on(async { browser.close().await.unwrap() });
}

fn extract_string(value: &RemoteValue) -> Option<String> {
    match value {
        RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::StringValue(sv)) => {
            Some(sv.value.clone())
        }
        _ => None,
    }
}

fn extract_number(value: &RemoteValue) -> Option<f64> {
    match value {
        RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::NumberValue(nv)) => {
            nv.value.as_f64()
        }
        _ => None,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// BROWSER LIFECYCLE
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_browser_lifecycle(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("browser_lifecycle");
    group.sample_size(10);

    group.bench_function("cold_start_and_close", |b| {
        b.iter(|| {
            let browser = launch_headless(&rt);
            teardown(browser, &rt);
        });
    });

    group.bench_function("cold_start_auto_attach", |b| {
        b.iter(|| {
            let browser = rt.block_on(async {
                let mut config = ChromeConfig::default();
                config.remote_debugging_port = Some(0);
                config.browser_flags = Some(vec![
                    "--headless=new".to_string(),
                    "--window-size=1280,720".to_string(),
                ]);
                create_chrome_browser(Some(config)).await
            });
            teardown(browser, &rt);
        });
    });

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
// NAVIGATION
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_navigation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("navigation");
    group.sample_size(10);

    let domains = [
        ("example.com", "https://example.com"),
        ("httpbin.org", "https://httpbin.org/html"),
        ("rust-lang.org", "https://www.rust-lang.org"),
        (
            "wikipedia",
            "https://en.wikipedia.org/wiki/Rust_(programming_language)",
        ),
        ("github", "https://github.com/nickel-org/nickel.rs"),
    ];

    for (name, url) in &domains {
        group.bench_with_input(BenchmarkId::new("navigate", name), url, |b, url| {
            let mut browser = launch_headless(&rt);
            b.iter(|| {
                nav_complete(&mut browser, black_box(url), &rt);
            });
            teardown(browser, &rt);
        });
    }

    group.bench_function("sequential_4_pages", |b| {
        let mut browser = launch_headless(&rt);
        let urls = [
            "https://example.com",
            "https://httpbin.org/html",
            "https://www.rust-lang.org",
            "https://en.wikipedia.org/wiki/Main_Page",
        ];
        b.iter(|| {
            for url in &urls {
                nav_complete(&mut browser, black_box(url), &rt);
            }
        });
        teardown(browser, &rt);
    });

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
// DOM QUERIES
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_dom_queries(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("dom_queries");
    group.sample_size(20);

    // Simple page queries
    {
        let mut browser = launch_headless(&rt);
        nav_complete(&mut browser, "https://example.com", &rt);

        group.bench_function("example_find_all", |b| {
            b.iter(|| {
                rt.block_on(async {
                    black_box(browser.find_nodes(css!("*")).await.unwrap());
                });
            });
        });

        group.bench_function("example_find_p_repeated", |b| {
            b.iter(|| {
                rt.block_on(async {
                    black_box(browser.find_nodes(css!("p")).await.unwrap());
                });
            });
        });

        teardown(browser, &rt);
    }

    // Complex page queries
    {
        let mut browser = launch_headless(&rt);
        nav_complete(
            &mut browser,
            "https://en.wikipedia.org/wiki/Rust_(programming_language)",
            &rt,
        );

        group.bench_function("wikipedia_find_all", |b| {
            b.iter(|| {
                rt.block_on(async {
                    black_box(browser.find_nodes(css!("*")).await.unwrap());
                });
            });
        });

        let selectors: &[(&str, &str)] = &[
            ("tag_a", "a"),
            ("tag_div", "div"),
            ("tag_span", "span"),
            ("class_reference", ".reference"),
            ("id_firstHeading", "#firstHeading"),
            ("compound_div_gt_p", "div > p"),
            ("attribute_a_href", "a[href]"),
            ("pseudo_first_child", ":first-child"),
        ];

        for (label, sel) in selectors {
            group.bench_with_input(
                BenchmarkId::new("wikipedia_selector", label),
                sel,
                |b, sel| {
                    b.iter(|| {
                        rt.block_on(async {
                            black_box(browser.find_nodes(css!(sel)).await.unwrap());
                        });
                    });
                },
            );
        }

        group.bench_function("wikipedia_find_node_h1", |b| {
            b.iter(|| {
                rt.block_on(async {
                    black_box(browser.find_node(css!("h1")).await.unwrap());
                });
            });
        });

        group.bench_function("wikipedia_wait_for_nodes_h1", |b| {
            b.iter(|| {
                rt.block_on(async {
                    black_box(browser.wait_for_nodes(css!("h1")).await.unwrap());
                });
            });
        });

        teardown(browser, &rt);
    }

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
// NODE PROPERTIES
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_node_properties(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("node_properties");
    group.sample_size(20);

    let mut browser = launch_headless(&rt);
    nav_complete(&mut browser, "https://example.com", &rt);

    group.bench_function("get_inner_text", |b| {
        let nodes = rt.block_on(browser.find_nodes(css!("h1"))).unwrap();
        b.iter(|| {
            rt.block_on(async { black_box(nodes[0].get_inner_text().await) });
        });
    });

    group.bench_function("get_text_content", |b| {
        let nodes = rt.block_on(browser.find_nodes(css!("body"))).unwrap();
        b.iter(|| {
            rt.block_on(async { black_box(nodes[0].get_text_content().await) });
        });
    });

    group.bench_function("get_inner_html", |b| {
        let nodes = rt.block_on(browser.find_nodes(css!("body"))).unwrap();
        b.iter(|| {
            rt.block_on(async { black_box(nodes[0].get_inner_html().await) });
        });
    });

    group.bench_function("get_attributes", |b| {
        let nodes = rt.block_on(browser.find_nodes(css!("a"))).unwrap();
        b.iter(|| {
            black_box(nodes[0].get_attributes());
        });
    });

    group.bench_function("get_position", |b| {
        let mut nodes = rt.block_on(browser.find_nodes(css!("h1"))).unwrap();
        b.iter(|| {
            rt.block_on(async { black_box(nodes[0].get_position().await) });
        });
    });

    group.bench_function("is_visible", |b| {
        let nodes = rt.block_on(browser.find_nodes(css!("h1"))).unwrap();
        b.iter(|| {
            rt.block_on(async { black_box(nodes[0].is_visible().await.unwrap()) });
        });
    });

    group.finish();
    teardown(browser, &rt);
}

// ═══════════════════════════════════════════════════════════════════════════════
// MOUSE INPUT
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_mouse_input(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("mouse_input");
    group.sample_size(20);

    let mut browser = launch_headless(&rt);
    nav_complete(&mut browser, "https://example.com", &rt);
    let context = browser.get_active_context_id().unwrap();

    group.bench_function("move_to", |b| {
        let mouse = browser.mouse();
        let mut i = 0u64;
        b.iter(|| {
            let x = (i as f64 * 13.7) % 800.0;
            let y = (i as f64 * 7.3) % 600.0;
            i += 1;
            rt.block_on(async {
                mouse
                    .move_to(Point { x, y }, &context, MouseMoveOptions::default())
                    .await
                    .unwrap();
            });
        });
    });

    group.bench_function("click", |b| {
        let mouse = browser.mouse();
        b.iter(|| {
            rt.block_on(async {
                mouse
                    .click(
                        Some(Point { x: 200.0, y: 200.0 }),
                        &context,
                        MouseClickOptions::default(),
                    )
                    .await
                    .unwrap();
            });
        });
    });

    group.bench_function("down_up_cycle", |b| {
        let mouse = browser.mouse();
        b.iter(|| {
            rt.block_on(async {
                mouse.down(&context, Default::default()).await.unwrap();
                mouse.up(&context, Default::default()).await.unwrap();
            });
        });
    });

    group.finish();
    teardown(browser, &rt);
}

// ═══════════════════════════════════════════════════════════════════════════════
// KEYBOARD INPUT
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_keyboard_input(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("keyboard_input");
    group.sample_size(10);

    let mut browser = launch_headless(&rt);
    nav_complete(&mut browser, "https://example.com", &rt);
    let context = browser.get_active_context_id().unwrap();

    let texts: &[(&str, &str)] = &[
        ("5_chars", "hello"),
        ("43_chars", "The quick brown fox jumps over the lazy dog"),
        (
            "150_chars",
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam.",
        ),
    ];

    for (label, text) in texts {
        group.bench_with_input(BenchmarkId::new("type_text", label), text, |b, text| {
            let keyboard = browser.keyboard();
            b.iter(|| {
                rt.block_on(async {
                    keyboard
                        .type_text(black_box(text), &context, None)
                        .await
                        .unwrap();
                });
            });
        });
    }

    group.bench_function("press_special_keys", |b| {
        let keyboard = browser.keyboard();
        let keys = [
            "Enter",
            "Tab",
            "Escape",
            "Backspace",
            "ArrowDown",
            "ArrowUp",
            "ArrowLeft",
            "ArrowRight",
            "Home",
            "End",
            "Delete",
            "F1",
            "F5",
            "F12",
        ];
        b.iter(|| {
            rt.block_on(async {
                for key in &keys {
                    keyboard
                        .press(black_box(key), &context, None)
                        .await
                        .unwrap();
                }
            });
        });
    });

    group.bench_function("modifier_combos", |b| {
        let keyboard = browser.keyboard();
        let modifiers = ["Shift", "Control", "Alt", "Meta"];
        b.iter(|| {
            rt.block_on(async {
                for modifier in &modifiers {
                    keyboard.down(modifier, &context).await.unwrap();
                    keyboard.press("KeyA", &context, None).await.unwrap();
                    keyboard.up(modifier, &context).await.unwrap();
                }
            });
        });
    });

    group.finish();
    teardown(browser, &rt);
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCRIPT EVALUATION
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_script_eval(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("script_eval");
    group.sample_size(20);

    let mut browser = launch_headless(&rt);
    nav_complete(&mut browser, "https://example.com", &rt);

    let simple_scripts: &[(&str, &str)] = &[
        ("arithmetic", "1 + 1"),
        ("string_concat", "'hello' + ' ' + 'world'"),
        ("boolean", "true && false"),
        ("null", "null"),
        ("array_length", "[1,2,3].length"),
        ("object_keys", "Object.keys({a:1, b:2}).length"),
        ("date_now", "Date.now()"),
        ("math_pi", "Math.PI"),
        (
            "json_roundtrip",
            "JSON.parse(JSON.stringify({a:1,b:[2,3]})).a",
        ),
    ];

    for (label, script) in simple_scripts {
        group.bench_with_input(BenchmarkId::new("simple", label), script, |b, script| {
            b.iter(|| {
                rt.block_on(async {
                    black_box(
                        browser
                            .evaluate_script(script.to_string(), false)
                            .await
                            .unwrap(),
                    );
                });
            });
        });
    }

    group.bench_function("repeated_1_plus_1", |b| {
        b.iter(|| {
            rt.block_on(async {
                black_box(
                    browser
                        .evaluate_script("1 + 1".to_string(), false)
                        .await
                        .unwrap(),
                );
            });
        });
    });

    let heavy_scripts: &[(&str, &str)] = &[
        (
            "sort_10k",
            "Array.from({length:10000}, (_,i)=>Math.random()).sort().length",
        ),
        (
            "fibonacci_30",
            "(function fib(n){return n<=1?n:fib(n-1)+fib(n-2)})(30)",
        ),
        ("regex_10k", "'a'.repeat(10000).match(/a/g).length"),
        (
            "json_1k_obj",
            "JSON.stringify(Object.fromEntries(Array.from({length:1000},(_,i)=>[`k${i}`,i]))).length",
        ),
    ];

    for (label, script) in heavy_scripts {
        group.bench_with_input(BenchmarkId::new("heavy", label), script, |b, script| {
            b.iter(|| {
                rt.block_on(async {
                    black_box(
                        browser
                            .evaluate_script(script.to_string(), false)
                            .await
                            .unwrap(),
                    );
                });
            });
        });
    }

    teardown(browser, &rt);

    // DOM reads on complex page
    let mut browser = launch_headless(&rt);
    nav_complete(
        &mut browser,
        "https://en.wikipedia.org/wiki/Rust_(programming_language)",
        &rt,
    );

    let dom_scripts: &[(&str, &str)] = &[
        ("document_title", "document.title"),
        ("location_href", "window.location.href"),
        (
            "all_elements_count",
            "document.querySelectorAll('*').length",
        ),
        ("links_count", "document.querySelectorAll('a').length"),
        ("body_text_length", "document.body.innerText.length"),
        ("computed_style", "getComputedStyle(document.body).fontSize"),
        ("scroll_height", "document.body.scrollHeight"),
    ];

    for (label, script) in dom_scripts {
        group.bench_with_input(BenchmarkId::new("dom_read", label), script, |b, script| {
            b.iter(|| {
                rt.block_on(async {
                    black_box(
                        browser
                            .evaluate_script(script.to_string(), false)
                            .await
                            .unwrap(),
                    );
                });
            });
        });
    }

    group.finish();
    teardown(browser, &rt);
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCREENSHOTS
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_screenshots(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("screenshots");
    group.sample_size(10);

    // Simple page
    {
        let mut browser = launch_headless(&rt);
        nav_complete(&mut browser, "https://example.com", &rt);

        group.bench_function("example_viewport", |b| {
            b.iter(|| {
                rt.block_on(async { black_box(browser.screenshot().await.unwrap()) });
            });
        });

        group.bench_function("example_node_h1", |b| {
            let nodes = rt.block_on(browser.find_nodes(css!("h1"))).unwrap();
            b.iter(|| {
                rt.block_on(async { black_box(nodes[0].screenshot().await.unwrap()) });
            });
        });

        teardown(browser, &rt);
    }

    // Complex page
    {
        let mut browser = launch_headless(&rt);
        nav_complete(
            &mut browser,
            "https://en.wikipedia.org/wiki/Rust_(programming_language)",
            &rt,
        );

        group.bench_function("wikipedia_viewport", |b| {
            b.iter(|| {
                rt.block_on(async { black_box(browser.screenshot().await.unwrap()) });
            });
        });

        teardown(browser, &rt);
    }

    // Cross-domain screenshots
    {
        let mut browser = launch_headless(&rt);
        let domains = [
            ("example_com", "https://example.com"),
            ("httpbin_org", "https://httpbin.org/html"),
            ("rust_lang_org", "https://www.rust-lang.org"),
        ];

        for (name, url) in &domains {
            nav_complete(&mut browser, url, &rt);
            group.bench_function(format!("cross_domain_{}", name), |b| {
                b.iter(|| {
                    rt.block_on(async { black_box(browser.screenshot().await.unwrap()) });
                });
            });
        }

        teardown(browser, &rt);
    }

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
// CONTEXT MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_context_management(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("context_management");
    group.sample_size(10);

    group.bench_function("create_context", |b| {
        let mut browser = launch_headless(&rt);
        b.iter(|| {
            rt.block_on(async {
                black_box(browser.create_context(true).await.unwrap());
            });
        });
        teardown(browser, &rt);
    });

    group.bench_function("create_5_contexts", |b| {
        let mut browser = launch_headless(&rt);
        b.iter(|| {
            rt.block_on(async {
                for _ in 0..5 {
                    black_box(browser.create_context(true).await.unwrap());
                }
            });
        });
        teardown(browser, &rt);
    });

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
// TIMEZONE EMULATION
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_timezone_emulation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("timezone_emulation");
    group.sample_size(10);

    let mut browser = launch_headless(&rt);
    nav_complete(&mut browser, "https://example.com", &rt);

    let timezones = [
        "America/New_York",
        "America/Los_Angeles",
        "Europe/London",
        "Europe/Berlin",
        "Asia/Tokyo",
        "Asia/Shanghai",
        "Australia/Sydney",
        "Pacific/Auckland",
    ];

    group.bench_function("cycle_8_timezones", |b| {
        b.iter(|| {
            rt.block_on(async {
                for tz in &timezones {
                    browser
                        .emulate_timezone(Some(tz.to_string()))
                        .await
                        .unwrap();
                }
            });
        });
    });

    rt.block_on(async { browser.emulate_timezone(None).await.unwrap() });
    group.finish();
    teardown(browser, &rt);
}

// ═══════════════════════════════════════════════════════════════════════════════
// END-TO-END FLOWS
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_end_to_end(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("end_to_end");
    group.sample_size(10);

    group.bench_function("full_flow_example_com", |b| {
        let mut browser = launch_headless(&rt);
        b.iter(|| {
            rt.block_on(async {
                browser
                    .navigate_with_options(
                        "https://example.com",
                        NavigateOptionsBuilder::default()
                            .wait(ReadinessState::Complete)
                            .build(),
                    )
                    .await
                    .unwrap();

                let nodes = browser.find_nodes(css!("*")).await.unwrap();
                black_box(nodes.len());

                let h1 = browser.find_nodes(css!("h1")).await.unwrap();
                black_box(h1[0].get_inner_text().await);

                black_box(browser.screenshot().await.unwrap());

                let eval = browser
                    .evaluate_script("document.title".to_string(), false)
                    .await
                    .unwrap();
                black_box(extract_string(&eval.result));

                let mut links = browser.find_nodes(css!("a")).await.unwrap();
                links[0].mouse_click().await.unwrap();

                let context = browser.get_active_context_id().unwrap();
                browser
                    .keyboard()
                    .type_text("rustenium", &context, None)
                    .await
                    .unwrap();
            });
        });
        teardown(browser, &rt);
    });

    group.bench_function("full_flow_wikipedia", |b| {
        let mut browser = launch_headless(&rt);
        b.iter(|| {
            rt.block_on(async {
                browser
                    .navigate_with_options(
                        "https://en.wikipedia.org/wiki/Rust_(programming_language)",
                        NavigateOptionsBuilder::default()
                            .wait(ReadinessState::Complete)
                            .build(),
                    )
                    .await
                    .unwrap();

                let nodes = browser.find_nodes(css!("*")).await.unwrap();
                black_box(nodes.len());

                let links = browser.find_nodes(css!("a")).await.unwrap();
                black_box(links.len());

                let h2s = browser.find_nodes(css!("h2")).await.unwrap();
                for h2 in &h2s {
                    black_box(h2.get_inner_text().await);
                }

                black_box(browser.screenshot().await.unwrap());

                let eval = browser
                    .evaluate_script(
                        "document.querySelectorAll('*').length".to_string(),
                        false,
                    )
                    .await
                    .unwrap();
                black_box(extract_number(&eval.result));
            });
        });
        teardown(browser, &rt);
    });

    group.bench_function("multi_tab_flow", |b| {
        let mut browser = launch_headless(&rt);
        let urls = [
            "https://example.com",
            "https://httpbin.org/html",
            "https://www.rust-lang.org",
        ];
        b.iter(|| {
            rt.block_on(async {
                let mut contexts = vec![browser.get_active_context_id().unwrap()];
                for _ in 1..urls.len() {
                    let ctx = browser.create_context(true).await.unwrap();
                    let id: String = ctx.into();
                    contexts.push(id.into());
                }

                for (url, ctx) in urls.iter().zip(contexts.iter()) {
                    browser
                        .navigate_with_options(
                            url,
                            NavigateOptionsBuilder::default().wait(ReadinessState::Complete).context_id(ctx.clone()).build()
                        )
                        .await
                        .unwrap();
                }

                for ctx in &contexts {
                    black_box(
                        browser
                            .screenshot_with_options(BrowserScreenshotOptionsBuilder::default().context_id(ctx.clone()).build()).await.unwrap()
                    );
                }
            });
        });
        teardown(browser, &rt);
    });

    group.finish();
}

// ═══════════════════════════════════════════════════════════════════════════════
// TRAJECTORY GENERATION (pure computation, no browser)
// ═══════════════════════════════════════════════════════════════════════════════

fn bench_trajectory_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("trajectory_generation");

    let distances: &[(&str, Point, Point)] = &[
        (
            "short_20px",
            Point { x: 100.0, y: 100.0 },
            Point { x: 120.0, y: 100.0 },
        ),
        (
            "medium_200px",
            Point { x: 100.0, y: 100.0 },
            Point { x: 300.0, y: 100.0 },
        ),
        (
            "long_800px",
            Point { x: 0.0, y: 0.0 },
            Point { x: 600.0, y: 500.0 },
        ),
        (
            "diagonal_1000px",
            Point { x: 0.0, y: 0.0 },
            Point { x: 707.0, y: 707.0 },
        ),
    ];

    for (label, from, to) in distances {
        group.bench_with_input(BenchmarkId::new("random_params", label), label, |b, _| {
            b.iter(|| black_box(random_curve_params(*from, *to)));
        });

        group.bench_with_input(BenchmarkId::new("full_trajectory", label), label, |b, _| {
            b.iter(|| {
                let params = random_curve_params(*from, *to);
                black_box(generate_trajectory(*from, *to, &params));
            });
        });
    }

    group.bench_function("trajectory_no_distortion", |b| {
        let from = Point { x: 0.0, y: 0.0 };
        let to = Point { x: 500.0, y: 300.0 };
        let params = CurveParams {
            offset_boundary_x: 50.0,
            offset_boundary_y: 50.0,
            knots_count: 2,
            distortion_mean: 0.0,
            distortion_stdev: 0.0,
            distortion_frequency: 0.0,
            tween: |t| t,
            target_points: 60,
        };
        b.iter(|| black_box(generate_trajectory(from, to, &params)));
    });

    group.bench_function("trajectory_heavy_distortion", |b| {
        let from = Point { x: 0.0, y: 0.0 };
        let to = Point { x: 500.0, y: 300.0 };
        let params = CurveParams {
            offset_boundary_x: 80.0,
            offset_boundary_y: 80.0,
            knots_count: 8,
            distortion_mean: 1.0,
            distortion_stdev: 1.0,
            distortion_frequency: 0.7,
            tween: |t| t,
            target_points: 80,
        };
        b.iter(|| black_box(generate_trajectory(from, to, &params)));
    });

    group.bench_function("generate_durations_50", |b| {
        b.iter(|| black_box(generate_durations(50, 0.6, (0.004, 0.025))));
    });

    group.bench_function("generate_durations_100", |b| {
        b.iter(|| black_box(generate_durations(100, 1.2, (0.004, 0.025))));
    });

    group.bench_function("swipe_trajectory_vertical", |b| {
        let from = Point { x: 200.0, y: 600.0 };
        let to = Point { x: 200.0, y: 100.0 };
        b.iter(|| {
            let params = random_curve_params(from, to);
            let traj = generate_trajectory(from, to, &params);
            black_box(generate_durations(traj.len(), 0.6, (0.004, 0.025)));
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_browser_lifecycle,
    bench_navigation,
    bench_dom_queries,
    bench_node_properties,
    bench_mouse_input,
    bench_keyboard_input,
    bench_script_eval,
    bench_screenshots,
    bench_context_management,
    bench_timezone_emulation,
    bench_end_to_end,
    bench_trajectory_generation,
);
criterion_main!(benches);
