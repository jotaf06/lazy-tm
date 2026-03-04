use crate::controler::events::AppEvent;
use std::io::Result;

pub trait InputReader {
    fn read(&mut self) -> Result<Option<AppEvent>>;  
    fn has_event(&self) -> Result<bool>;
    fn name(&self) -> &str;
}

pub trait TimedInputReader: InputReader {
    fn set_timeout(&mut self, timeout_ms: u64);
    fn timeout(&self) -> u64;
    fn read_with_timeout(&mut self) -> Result<Option<AppEvent>> {
        if !self.has_event()? {
            return Ok(None);
        }
        self.read()
    }
}

pub struct InputAdapter<R: InputReader> {
    reader: R,
}

impl<R: InputReader> InputAdapter<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
    pub fn read_event(&mut self) -> Result<Option<AppEvent>> {
        self.reader.read()
    }
    pub fn inner(&self) -> &R {
        &self.reader
    }
    pub fn inner_mut(&mut self) -> &mut R {
        &mut self.reader
    }
}

pub struct CompositeInputReader {
    readers: Vec<Box<dyn InputReader>>,
}

impl CompositeInputReader {
    pub fn new() -> Self {
        Self {
            readers: Vec::new(),
        }
    }

    pub fn add_reader(&mut self, reader: Box<dyn InputReader>) {
        self.readers.push(reader);
    }

    pub fn clear(&mut self) {
        self.readers.clear();
    }

    pub fn len(&self) -> usize {
        self.readers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.readers.is_empty()
    }
}

impl InputReader for CompositeInputReader {
    fn read(&mut self) -> Result<Option<AppEvent>> {
        for reader in &mut self.readers {
            if let Some(event) = reader.read()? {
                return Ok(Some(event));
            }
        }
        Ok(None)
    }

    fn has_event(&self) -> Result<bool> {
        for reader in &self.readers {
            if reader.has_event()? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn name(&self) -> &str {
        "CompositeInputReader"
    }
}

impl Default for CompositeInputReader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockReader {
        events: Vec<AppEvent>,
    }

    impl MockReader {
        fn new(events: Vec<AppEvent>) -> Self {
            Self { events }
        }
    }

    impl InputReader for MockReader {
        fn read(&mut self) -> Result<Option<AppEvent>> {
            Ok(self.events.pop())
        }

        fn has_event(&self) -> Result<bool> {
            Ok(!self.events.is_empty())
        }

        fn name(&self) -> &str {
            "MockReader"
        }
    }

    #[test]
    fn test_composite_reader() {
        let mut composite = CompositeInputReader::new();
        composite.add_reader(Box::new(MockReader::new(vec![AppEvent::Quit])));
        composite.add_reader(Box::new(MockReader::new(vec![AppEvent::Add])));

        assert!(composite.has_event().unwrap());
        assert!(composite.read().unwrap().is_some());
    }
}
