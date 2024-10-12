use anyhow::Result;
use palette::rgb::Rgb;

use photonic::boxed::{BoxedOutput, BoxedOutputDecl};
use photonic::{BufferReader, Output, OutputDecl};

#[derive(Default)]
pub struct Split {
    outputs: Vec<BoxedOutputDecl>,
}

impl Split {
    pub fn new(outputs: Vec<BoxedOutputDecl>) -> Self {
        return Self {
            outputs,
        };
    }
}

impl OutputDecl for Split {
    const KIND: &'static str = "split";
    type Output = SplitOutput;

    async fn materialize(self) -> Result<Self::Output>
    where Self::Output: Sized {
        let mut size = 0usize;

        let mut outputs = Vec::new();
        for output in self.outputs {
            let output = output.materialize().await?;
            let offset = size;

            size += output.size();

            outputs.push((offset, output));
        }

        return Ok(Self::Output {
            size,
            outputs,
        });
    }
}

pub struct SplitOutput {
    size: usize,
    outputs: Vec<(usize, BoxedOutput)>,
}

impl Output for SplitOutput {
    const KIND: &'static str = "null";

    type Element = Rgb;

    async fn render(&mut self, buf: impl BufferReader<Element = Self::Element>) -> Result<()> {
        for (offset, output) in self.outputs.iter_mut() {
            let range = (*offset)..(*offset + output.size());
            output.render(buf.slice(range)).await?;
        }

        return Ok(());
    }

    fn size(&self) -> usize {
        return self.size;
    }
}
