use ::clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Process a Payment
    Pay {
        /// The provider to use
        #[arg(short, long)]
        provider: String,
    },
    
    /// Arc and Mutex concurrency
    Concurrency {}
}
