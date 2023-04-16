fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/auth.proto")?;
    tonic_build::compile_protos("proto/comment.proto")?;
    tonic_build::compile_protos("proto/email.proto")?;
    tonic_build::compile_protos("proto/image.proto")?;
    tonic_build::compile_protos("proto/meme.proto")?;
    tonic_build::compile_protos("proto/notification.proto")?;
    tonic_build::compile_protos("proto/profile.proto")?;
    tonic_build::compile_protos("proto/votes.proto")?;
    Ok(())
}