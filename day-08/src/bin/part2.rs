use day_08::part2::solve;

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input2.txt");
    let result = solve(file)?;
    println!("SOLUTION: {}", result);
    Ok(())
}