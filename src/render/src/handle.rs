use std::sync::mpsc;

use memory::Typed;
use Backend;

pub(crate) type GarbageSender<B> = mpsc::Sender<Garbage<B>>;
pub(crate) type GarbageReceiver<B> = mpsc::Receiver<Garbage<B>>;
pub(crate) fn garbage_channel<B: Backend>() -> (GarbageSender<B>, GarbageReceiver<B>) {
    mpsc::channel()
}

macro_rules! define_resources {
    ($($name:ident: $info:path,)*) => {
        pub enum Garbage<B: Backend> {
            $( $name(B::$name), )*
        }

        #[derive(Clone)]
        pub enum Any<B: Backend> {
            $( $name(self::raw::$name<B>), )*
        }

        pub mod inner {
            use Backend;
            use super::{Garbage, GarbageSender};
            use std::{cmp, hash};

            $(
            
            #[derive(Debug)]
            pub struct $name<B: Backend> {
                // option for owned drop
                resource: Option<B::$name>,
                info: $info,
                garbage: GarbageSender<B>
            }

            impl<B: Backend> $name<B> {
                pub(crate) fn new(
                    resource: B::$name,
                    info: $info,
                    garbage: GarbageSender<B>) -> Self
                {
                    $name {
                        resource: Some(resource),
                        info,
                        garbage,
                    }
                }

                pub fn resource(&self) -> &B::$name {
                    self.resource.as_ref().unwrap()
                }

                pub fn info(&self) -> &$info {
                    &self.info
                }
            }

            impl<B: Backend> cmp::PartialEq for $name<B>
                where B::$name: cmp::PartialEq
            {
                fn eq(&self, other: &$name<B>) -> bool {
                    self.resource().eq(&other.resource())
                }
            }

            impl<B: Backend> cmp::Eq for $name<B>
                where B::$name: cmp::Eq
            {}

            impl<B: Backend> hash::Hash for $name<B>
                where B::$name: hash::Hash
            {
                fn hash<H: hash::Hasher>(&self, state: &mut H) {
                    self.resource().hash(state)
                }
            }

            impl<B: Backend> Drop for $name<B> {
                fn drop(&mut self) {
                    let res = self.resource.take().unwrap();
                    self.garbage.send(Garbage::$name(res))
                        .unwrap_or_else(|e|
                            error!("Could not drop {}: {}", stringify!($name), e));
                }
            }

            )*
        }

        pub mod raw {
            use std::sync::Arc;
            use Backend;
            $(
                pub type $name<B> = Arc<super::inner::$name<B>>;

                impl<B: Backend> From<$name<B>> for super::Any<B> {
                    fn from(h: $name<B>) -> Self {
                        super::Any::$name(h)
                    }
                }
            )*
        }
    }
}

define_resources! {
    // Heap
    // ShaderLib,
    // RenderPass
    // PipelineLayout
    // GraphicsPipeline
    // ComputePipeline
    // FrameBuffer
    Buffer: ::buffer::Info,
    Image: ::image::Info,
    RenderTargetView: ::handle::ViewSource<B>,
    DepthStencilView: ::handle::ViewSource<B>,
    ConstantBufferView: ::handle::raw::Buffer<B>,
    ShaderResourceView: ::handle::ViewSource<B>,
    UnorderedAccessView: ::handle::ViewSource<B>,
    Sampler: ::image::SamplerInfo,
    // DescriptorPool
    // DescriptorSetLayout
    // Fence
    // Semaphore
}

pub type Buffer<B, T> = Typed<raw::Buffer<B>, T>;
pub type Image<B, F> = Typed<raw::Image<B>, F>;
pub type RenderTargetView<B, F> = Typed<raw::RenderTargetView<B>, F>;
pub type DepthStencilView<B, F> = Typed<raw::DepthStencilView<B>, F>;
pub type ConstantBufferView<B, T> = Typed<raw::ConstantBufferView<B>, T>;
pub type ShaderResourceView<B, T> = Typed<raw::ShaderResourceView<B>, T>;
pub type UnorderedAccessView<B, T> = Typed<raw::UnorderedAccessView<B>, T>;

pub use self::raw::Sampler;

#[derive(Debug, Clone)]
pub enum ViewSource<B: Backend> {
    Image(raw::Image<B>),
    Buffer(raw::Buffer<B>),
    Backbuffer(B::Image, ::image::Info),
}

impl<'a, B: Backend> From<&'a raw::Image<B>> for ViewSource<B> {
    fn from(image: &'a raw::Image<B>) -> Self {
        ViewSource::Image(image.clone())
    }
}

impl<'a, B: Backend> From<&'a raw::Buffer<B>> for ViewSource<B> {
    fn from(buffer: &'a raw::Buffer<B>) -> Self {
        ViewSource::Buffer(buffer.clone())
    }
}

pub(crate) struct Bag<B: Backend>(Vec<Any<B>>);

impl<B: Backend> Bag<B> {
    pub fn new() -> Self {
        Bag(Vec::new())
    }

    pub fn add<H: Into<Any<B>>>(&mut self, handle: H) {
        self.0.push(handle.into());
    }

    pub fn extend(&mut self, other: &Bag<B>) {
        self.0.extend_from_slice(&other.0);
    }

    pub fn append(&mut self, other: &mut Bag<B>) {
        self.0.append(&mut other.0);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}
