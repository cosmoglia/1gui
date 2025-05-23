use std::borrow::Cow;

use super::{
    BytesLoader as _, Context, HashMap, ImagePoll, Mutex, SizeHint, SizedTexture, TextureHandle,
    TextureLoadResult, TextureLoader, TextureOptions, TexturePoll,
};

#[derive(Default)]
pub struct DefaultTextureLoader {
    cache: Mutex<HashMap<(Cow<'static, str>, TextureOptions), TextureHandle>>,
}

impl TextureLoader for DefaultTextureLoader {
    fn id(&self) -> &'static str {
        crate::generate_loader_id!(DefaultTextureLoader)
    }

    fn load(
        &self,
        ctx: &Context,
        uri: &str,
        texture_options: TextureOptions,
        size_hint: SizeHint,
    ) -> TextureLoadResult {
        let mut cache = self.cache.lock();
        if let Some(handle) = cache.get(&(Cow::Borrowed(uri), texture_options)) {
            let texture = SizedTexture::from_handle(handle);
            Ok(TexturePoll::Ready { texture })
        } else {
            match ctx.try_load_image(uri, size_hint)? {
                ImagePoll::Pending { size } => Ok(TexturePoll::Pending { size }),
                ImagePoll::Ready { image } => {
                    let handle = ctx.load_texture(uri, image, texture_options);
                    let texture = SizedTexture::from_handle(&handle);
                    cache.insert((Cow::Owned(uri.to_owned()), texture_options), handle);
                    let reduce_texture_memory = ctx.options(|o| o.reduce_texture_memory);
                    if reduce_texture_memory {
                        let loaders = ctx.loaders();
                        loaders.include.forget(uri);
                        for loader in loaders.bytes.lock().iter().rev() {
                            loader.forget(uri);
                        }
                        for loader in loaders.image.lock().iter().rev() {
                            loader.forget(uri);
                        }
                    }
                    Ok(TexturePoll::Ready { texture })
                }
            }
        }
    }

    fn forget(&self, uri: &str) {
        #[cfg(feature = "log")]
        log::trace!("forget {uri:?}");

        self.cache.lock().retain(|(u, _), _| u != uri);
    }

    fn forget_all(&self) {
        #[cfg(feature = "log")]
        log::trace!("forget all");

        self.cache.lock().clear();
    }

    fn end_pass(&self, _: usize) {}

    fn byte_size(&self) -> usize {
        self.cache
            .lock()
            .values()
            .map(|texture| texture.byte_size())
            .sum()
    }
}
