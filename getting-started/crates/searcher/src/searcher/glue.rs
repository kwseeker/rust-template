
#[derive(Debug)]
pub(crate) struct ReadByLine<'s, M, R, S> {
    config: &'s Config,
    core: Core<'s, M, S>,
    rdr: LineBufferReader<'s, R>,
}

impl<'s, M, R, S> ReadByLine<'s, M, R, S>
where
    M: Matcher,
    R: std::io::Read,
    S: Sink,
{
    pub(crate) fn run(mut self) -> Result<(), S::Error> {
        
    }
}