use std::time::Instant;

pub trait Handler {
    /// Associated input event type
    type Ein: 'static;
    /// Associated output event type
    type Eout: 'static;
    /// Associated input read type
    type Rin: 'static;
    /// Associated output read type
    type Rout: 'static;
    /// Associated input write type
    type Win: 'static;
    /// Associated output write type
    type Wout: 'static;
    /// Associated result error type
    type Error: 'static;

    /// Handles Rin and returns Rout for next inbound handler handling
    fn handle_read(&mut self, msg: Self::Rin) -> Result<(), Self::Error>;

    /// Polls Rout from internal queue for next inbound handler handling
    fn poll_read(&mut self) -> Option<Self::Rout>;

    /// Handles Win and returns Wout for next outbound handler handling
    fn handle_write(&mut self, msg: Self::Win) -> Result<(), Self::Error>;

    /// Polls Wout from internal queue for next outbound handler handling
    fn poll_write(&mut self) -> Option<Self::Wout>;

    /// Handles event
    fn handle_event(&mut self, _evt: Self::Ein) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Polls event
    fn poll_event(&mut self) -> Option<Self::Eout> {
        None
    }

    /// Handles timeout
    fn handle_timeout(&mut self, _now: Instant) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Polls timeout
    fn poll_timeout(&mut self) -> Option<Instant> {
        None
    }
}
