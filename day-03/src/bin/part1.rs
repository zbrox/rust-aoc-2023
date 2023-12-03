use day_03::part1::solve;

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1.txt");
    let result = solve(file)?;
    println!("SOLUTION: {}", result);
    Ok(())
}