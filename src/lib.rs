#![feature(test)]

extern crate test;

#[cfg(feature = "protobuf-codec")]
mod protobuf;
mod text;

#[cfg(feature = "protobuf-codec")]
pub use crate::protobuf::*;

#[cfg(feature = "prost-codec")]
pub use crate::prost::*;

pub mod tests {
    use test::Bencher;
    use crate::raft_cmdpb::RaftCmdResponse;

    fn verify_current_term(msg: RaftCmdResponse) {
        assert_eq!(msg.get_header().get_current_term(), 2);
    }

    pub fn clone_and_move(msg: &RaftCmdResponse) {
        let m = msg.clone();
        verify_current_term(m);
    }

    #[bench]
    fn bench_move_raft_cmd_response(b: &mut Bencher) {
        let mut msg = RaftCmdResponse::new();
        msg.mut_header().set_current_term(2);
        b.iter(|| {
            clone_and_move(&msg);
        });
    }
}

