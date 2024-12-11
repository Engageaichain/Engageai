use std::error::Error;
use tokio::runtime::Runtime;

mod blockchain;
mod social_validator;
mod network;
mod engagement;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize MediaChain node
    let node = blockchain::MediaChainNode::new()?;
    
    // Start network service
    let network_service = network::NetworkService::new(node.clone());
    network_service.start().await?;

    // Initialize social engagement validator
    let engagement_validator = engagement::EngagementValidator::new();
    
    // Main event loop
    loop {
        // Process pending transactions
        node.process_transactions(&engagement_validator).await?;
        
        // Sync with network
        network_service.sync().await?;
        
        // Short sleep to prevent CPU overload
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
