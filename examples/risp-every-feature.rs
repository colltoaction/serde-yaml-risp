extern crate risp;

use risp::core::create_core_environment;
use risp::types::RispResult;

use risp::environment::Environment;
use risp::eval::eval;
use serde_yaml::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use std::error::Error;

pub fn read_term_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_yaml::from_reader(reader)?)
}

pub fn eval_yaml_risp_script(path: &Path, env: &mut Environment) -> RispResult {
    let file = read_term_from_file(path).unwrap();

    let file_risp = serde_yaml_risp::convert(&file);
    eval(file_risp, env)
}

fn main() {
    let mut env = create_core_environment();

    let r0 = eval_yaml_risp_script(Path::new("examples/risp-0.yaml"), &mut env);
    println!("{:?}", r0);

    let r1 = eval_yaml_risp_script(Path::new("examples/risp-1.yaml"), &mut env);
    println!("{:?}", r1);

    let r2 = eval_yaml_risp_script(Path::new("examples/risp-2.yaml"), &mut env);
    println!("{:?}", r2);

    let r3 = eval_yaml_risp_script(Path::new("examples/risp-3.yaml"), &mut env);
    println!("{:?}", r3);

    let r4 = eval_yaml_risp_script(Path::new("examples/risp-4.yaml"), &mut env);
    println!("{:?}", r4);

    let r5 = eval_yaml_risp_script(Path::new("examples/risp-5.yaml"), &mut env);
    println!("{:?}", r5);

    let r6 = eval_yaml_risp_script(Path::new("examples/risp-6.yaml"), &mut env);
    println!("{:?}", r6);

    let r7 = eval_yaml_risp_script(Path::new("examples/risp-7-defn.yaml"), &mut env);
    println!("{:?}", r7);

    let r8 = eval_yaml_risp_script(Path::new("examples/risp-8.yaml"), &mut env);
    println!("{:?}", r8);
}
