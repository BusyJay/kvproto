#![feature(test)]

extern crate test;

pub use crate::prost::*;

mod prost;

pub mod prost_adapt {
    use crate::import_kvpb::{write_engine_request, WriteBatch, WriteEngineRequest, WriteHead};
    use crate::import_sstpb::{upload_request, SstMeta, UploadRequest};
    impl UploadRequest {
        pub fn set_data(&mut self, v: Vec<u8>) {
            self.chunk = Some(upload_request::Chunk::Data(v));
        }
        pub fn get_data(&self) -> &[u8] {
            match &self.chunk {
                Some(upload_request::Chunk::Data(v)) => v,
                _ => &[],
            }
        }
        pub fn set_meta(&mut self, v: SstMeta) {
            self.chunk = Some(upload_request::Chunk::Meta(v));
        }
        pub fn get_meta(&self) -> &SstMeta {
            match &self.chunk {
                Some(upload_request::Chunk::Meta(v)) => v,
                _ => <SstMeta as ::protobuf::Message>::default_instance(),
            }
        }
        pub fn has_meta(&self) -> bool {
            match self.chunk {
                Some(upload_request::Chunk::Meta(_)) => true,
                _ => false,
            }
        }
    }

    impl WriteEngineRequest {
        pub fn set_head(&mut self, v: WriteHead) {
            self.chunk = Some(write_engine_request::Chunk::Head(v));
        }
        pub fn get_head(&self) -> &WriteHead {
            match &self.chunk {
                Some(write_engine_request::Chunk::Head(v)) => v,
                _ => <WriteHead as ::protobuf::Message>::default_instance(),
            }
        }
        pub fn has_head(&self) -> bool {
            match self.chunk {
                Some(write_engine_request::Chunk::Head(_)) => true,
                _ => false,
            }
        }
        pub fn set_batch(&mut self, v: WriteBatch) {
            self.chunk = Some(write_engine_request::Chunk::Batch(v));
        }
        pub fn get_batch(&self) -> &WriteBatch {
            match &self.chunk {
                Some(write_engine_request::Chunk::Batch(v)) => v,
                _ => <WriteBatch as ::protobuf::Message>::default_instance(),
            }
        }
        pub fn has_batch(&self) -> bool {
            match self.chunk {
                Some(write_engine_request::Chunk::Batch(_)) => true,
                _ => false,
            }
        }
        pub fn take_batch(&mut self) -> WriteBatch {
            if self.has_batch() {
                match self.chunk.take() {
                    Some(write_engine_request::Chunk::Batch(v)) => v,
                    _ => unreachable!(),
                }
            } else {
                WriteBatch::new_()
            }
        }
    }
}

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
        let mut msg = RaftCmdResponse::new_();
        msg.mut_header().set_current_term(2);
        b.iter(|| {
            clone_and_move(&msg);
        });
    }
}

