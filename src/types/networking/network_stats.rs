//! Network statistics for monitoring connection quality

use crate::BinarySerializable;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Network statistics for monitoring connection quality
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NetworkStats {
    /// Packets sent per second
    pub packets_out_per_second: f32,
    /// Packets received per second
    pub packets_in_per_second: f32,
    /// Bytes sent per second
    pub bytes_out_per_second: f32,
    /// Bytes received per second
    pub bytes_in_per_second: f32,
    /// Round trip time in milliseconds
    pub ping: f32,
    /// Packet loss percentage (0.0 to 1.0)
    pub packet_loss: f32,
    /// Network jitter in milliseconds
    pub jitter: f32,
    /// Connection uptime in seconds
    pub uptime: f32,
}

impl fmt::Display for NetworkStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NetworkStats(Ping: {:.1}ms, Loss: {:.2}%, Jitter: {:.1}ms, Out: {:.1} B/s, In: {:.1} B/s)",
            self.ping,
            self.packet_loss * 100.0,
            self.jitter,
            self.bytes_out_per_second,
            self.bytes_in_per_second
        )
    }
}

impl BinarySerializable for NetworkStats {}

impl NetworkStats {
    /// Create new network stats with default values
    pub fn new() -> Self {
        Self {
            packets_out_per_second: 0.0,
            packets_in_per_second: 0.0,
            bytes_out_per_second: 0.0,
            bytes_in_per_second: 0.0,
            ping: 0.0,
            packet_loss: 0.0,
            jitter: 0.0,
            uptime: 0.0,
        }
    }

    /// Check if the connection quality is good
    pub fn is_connection_good(&self) -> bool {
        self.ping < 100.0 && self.packet_loss < 0.05 && self.jitter < 50.0
    }

    /// Get connection quality as a value from 0.0 (worst) to 1.0 (best)
    pub fn connection_quality(&self) -> f32 {
        let ping_score = (1.0 - (self.ping / 200.0).min(1.0)).max(0.0);
        let loss_score = (1.0 - (self.packet_loss * 20.0).min(1.0)).max(0.0);
        let jitter_score = (1.0 - (self.jitter / 100.0).min(1.0)).max(0.0);
        
        (ping_score + loss_score + jitter_score) / 3.0
    }
}

impl Default for NetworkStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_stats() {
        let mut stats = NetworkStats::new();
        stats.ping = 40.0;
        stats.packet_loss = 0.01; // 1%
        stats.jitter = 20.0;
        
        assert!(stats.is_connection_good());
        assert!(stats.connection_quality() > 0.7);
        
        stats.ping = 150.0;
        stats.packet_loss = 0.1; // 10%
        assert!(!stats.is_connection_good());
    }

    #[test]
    fn test_display_formatting() {
        let stats = NetworkStats {
            ping: 50.0,
            packet_loss: 0.05,
            jitter: 25.0,
            bytes_out_per_second: 1024.0,
            bytes_in_per_second: 2048.0,
            ..Default::default()
        };
        
        let display_str = format!("{}", stats);
        assert!(display_str.contains("50.0ms"));
        assert!(display_str.contains("5.00%"));
        assert!(display_str.contains("25.0ms"));
    }
}