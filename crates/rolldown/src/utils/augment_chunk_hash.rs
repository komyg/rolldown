use super::chunk::render_chunk::ChunkRenderReturn;
use anyhow::Result;
use futures::future::try_join_all;
use rolldown_plugin::SharedPluginDriver;

#[tracing::instrument(level = "debug", skip_all)]
pub async fn augment_chunk_hash<'a>(
  plugin_driver: &SharedPluginDriver,
  chunks: Vec<ChunkRenderReturn>,
) -> Result<Vec<ChunkRenderReturn>> {
  try_join_all(chunks.into_iter().map(|chunk| async move {
    plugin_driver.augment_chunk_hash(&chunk.rendered_chunk).await.map(|augment_chunk_hash| {
      ChunkRenderReturn {
        code: chunk.code,
        map: chunk.map,
        rendered_chunk: chunk.rendered_chunk,
        augment_chunk_hash,
        file_dir: chunk.file_dir,
        preliminary_filename: chunk.preliminary_filename,
      }
    })
  }))
  .await
}
