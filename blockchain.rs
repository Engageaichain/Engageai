use sha2::{Sha256, Digest};
use ed25519_dalek::Keypair;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct MediaTransaction {
    pub transaction_id: String,
    pub sender: Vec<u8>,
    pub receiver: Vec<u8>,
    pub payload: Vec<u8>,
    pub social_proof: SocialEngagementMetrics,
    pub validation_timestamp: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SocialEngagementMetrics {
    pub platform_scores: std::collections::HashMap<String, f64>,
    pub total_reach: u64,
    pub interaction_quality: f64,
    pub geographical_distribution: Vec<String>,
}

pub struct MediaChainNode {
    keypair: Keypair,
    blockchain: Vec<MediaTransaction>,
    pending_transactions: Vec<MediaTransaction>,
}

impl MediaChainNode {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut rng = rand::rngs::OsRng;
        let keypair = Keypair::generate(&mut rng);
        
        Ok(Self {
            keypair,
            blockchain: Vec::new(),
            pending_transactions: Vec::new(),
        })
    }

    pub fn create_transaction(
        &mut self, 
        receiver: Vec<u8>, 
        payload: Vec<u8>
    ) -> MediaTransaction {
        let mut hasher = Sha256::new();
        hasher.update(&payload);
        let transaction_id = format!("{:x}", hasher.finalize());

        MediaTransaction {
            transaction_id,
            sender: self.keypair.public.to_bytes().to_vec(),
            receiver,
            payload,
            social_proof: SocialEngagementMetrics {
                platform_scores: std::collections::HashMap::new(),
                total_reach: 0,
                interaction_quality: 0.0,
                geographical_distribution: Vec::new(),
            },
            validation_timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }

    pub async fn process_transactions(
        &mut self, 
        engagement_validator: &engagement::EngagementValidator
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut validated_transactions = Vec::new();

        for transaction in &self.pending_transactions {
            if engagement_validator.validate(transaction).await? {
                validated_transactions.push(transaction.clone());
            }
        }

        // Add validated transactions to blockchain
        self.blockchain.extend(validated_transactions);
        
        Ok(())
    }
}
