use crate::workspace::prepare_working_directory;
use crate::{
    image::{are_images_equal, read_image_from_file, save_screen_as_img_png},
    workspace::{TEST_FILE_EXTENSION, TEST_RESULTS_DIR, TEST_TEMPLATE_DIR},
};
use basic_2d_geometries::{
    test_2d_default_color_on_default_background, test_2d_red_triangle_on_green_background,
    test_2d_two_triangles_green_blue,
};
use colored::Colorize;
use phoenix::{
    renderer::{opengl::OpenGL, Render},
    window::{GlfwConfig, Resolution, Window},
};
use std::rc::Rc;

pub mod basic_2d_geometries;

type TestFunction = fn(Rc<Window>, Box<dyn Render>);
type TestName = &'static str;

static TESTS: [(TestFunction, TestName); 3] = [
    (
        test_2d_red_triangle_on_green_background,
        "test_2d_red_triangle_on_green_background",
    ),
    (
        test_2d_default_color_on_default_background,
        "test_2d_default_color_on_default_background",
    ),
    (
        test_2d_two_triangles_green_blue,
        "test_2d_two_triangles_green_blue",
    ),
];

pub fn run() {
    prepare_working_directory();
    let config = create_config();
    let renderer = Box::<OpenGL>::default();
    let mut failed_tests = Vec::new();
    let mut passed_tests = Vec::new();

    println!("running opengl tests");
    for test in TESTS {
        if run_specific_test(&config, renderer.clone(), test.0, test.1) {
            passed_tests.push(test.1);
        } else {
            failed_tests.push(test.1);
        }
    }

    print_tests_status(failed_tests, passed_tests);
}

pub fn run_specific_test(
    config: &GlfwConfig,
    renderer: Box<dyn Render>,
    run_test: fn(Rc<Window>, Box<dyn Render>),
    test_name: &str,
) -> bool {
    let window = Rc::new(create_window(config));
    run_test(window.clone(), renderer);
    let result_path = TEST_RESULTS_DIR.to_owned() + test_name + TEST_FILE_EXTENSION;
    let template_path = TEST_TEMPLATE_DIR.to_owned() + test_name + TEST_FILE_EXTENSION;
    save_screen_as_img_png(window.as_ref(), &result_path).unwrap();

    if let Ok(result_image) = read_image_from_file(&result_path) {
        if let Ok(template_image) = read_image_from_file(&template_path) {
            are_images_equal(&result_image, &template_image)
        } else {
            println!(
                "Failed to read test template image from path: {}",
                template_path
            );
            false
        }
    } else {
        println!(
            "Failed to read test result image from path: {}",
            result_path
        );
        false
    }
}

fn print_tests_status(failed_tests: Vec<TestName>, passed_tests: Vec<TestName>) {
    for test in failed_tests {
        println!("test {} {}", test, "FAILED".red());
    }
    for test in passed_tests {
        println!("test {} {}", test, "PASSED".green());
    }
}

fn create_config() -> GlfwConfig {
    GlfwConfig::create().unwrap()
}

fn create_window(config: &GlfwConfig) -> Window {
    config
        .create_window(
            "Test",
            Resolution {
                width: 800,
                height: 600,
            },
        )
        .unwrap()
}
