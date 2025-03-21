use clap::{Subcommand, ValueEnum, Args};

use crate::rbx::{RbxCloud, PublishVersionType};

#[derive(Debug, Subcommand)]
pub enum ExperienceCommands {
    Publish {
        /// Filename (full or relative) of the RBXL file
        #[clap(short, long, value_parser)]
        filename: String,

        /// Place ID of the experience
        #[clap(short, long, value_parser)]
        place_id: u64,
    
        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Version type
        #[clap(short, long, value_enum)]
        version_type: VersionType,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser)]
        api_key: String,
    },
}

#[derive(Debug, Args)]
pub struct Experience {
    #[clap(subcommand)]
    command: ExperienceCommands,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum VersionType {
    Saved,
    Published,
}

impl Experience {
	pub async fn run(self) -> anyhow::Result<Option<String>> {
		match self.command {
			ExperienceCommands::Publish {place_id,universe_id,version_type,api_key, filename } => {
				let rbx_cloud = RbxCloud::new(api_key, universe_id);
				let publish_version_type = match version_type {
					VersionType::Published => PublishVersionType::Published,
					VersionType::Saved => PublishVersionType::Saved,
				};
				let res = rbx_cloud.experience(place_id).publish(&filename, publish_version_type).await;
				match res {
					Ok(body) => {
						Ok(Some(format!("{:?} {}/{} with version number {}", version_type, universe_id, place_id, body.version_number).to_lowercase()))
					}
					Err(err) => {
						Err(err)
					}
				}
			}
		}
	}
}
