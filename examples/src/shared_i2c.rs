use core::{fmt::Debug, future::Future};
use embassy::blocking_mutex::raw::RawMutex;
use embassy::mutex::Mutex;

pub use embedded_hal::i2c::{
    Error, ErrorKind, ErrorType,
};
use embedded_hal_async::i2c;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum I2cBusDeviceError<BUS> {
    I2c(BUS),
}

impl<BUS> i2c::Error for I2cBusDeviceError<BUS>
where
    BUS: i2c::Error + Debug,
{
    fn kind(&self) -> i2c::ErrorKind {
        match self {
            Self::I2c(e) => e.kind(),
        }
    }
}

pub struct I2cBusDevice<'a, M: RawMutex, BUS> {
    bus: &'a Mutex<M, BUS>,
}

impl<'a, M: RawMutex, BUS> I2cBusDevice<'a, M, BUS> {
    pub fn new(bus: &'a Mutex<M, BUS>) -> Self {
        Self { bus }
    }
}

impl<'a, M: RawMutex, BUS> i2c::ErrorType for I2cBusDevice<'a, M, BUS>
where
    BUS: i2c::ErrorType,
{
    type Error = I2cBusDeviceError<BUS::Error>;
}

impl<M, BUS> i2c::I2c for I2cBusDevice<'_, M, BUS>
where
    M: RawMutex + 'static,
    BUS: i2c::I2c + 'static, 
{
    type ReadFuture<'a> = impl Future<Output = Result<(), Self::Error>> + 'a where Self: 'a;

    fn read<'a>(&'a mut self, address: u8, buffer: &'a mut [u8]) -> Self::ReadFuture<'a> {
        async move {
            let mut bus = self.bus.lock().await;
            bus.read(address, buffer).await.map_err(I2cBusDeviceError::I2c)?;
            Ok(())
        }
    }

    type WriteFuture<'a> = impl Future<Output = Result<(), Self::Error>> + 'a where Self: 'a;

    fn write<'a>(&'a mut self, address: u8, bytes: &'a [u8]) -> Self::WriteFuture<'a> {
        async move {
            let mut bus = self.bus.lock().await;
            bus.write(address, bytes).await.map_err(I2cBusDeviceError::I2c)?;
            Ok(())
        }
    }

    type WriteReadFuture<'a> = impl Future<Output = Result<(), Self::Error>> + 'a where Self: 'a;

    fn write_read<'a>(
        &'a mut self,
        address: u8,
        wr_buffer: &'a [u8],
        rd_buffer: &'a mut [u8],
    ) -> Self::WriteReadFuture<'a> {
        async move {
            let mut bus = self.bus.lock().await;
            bus.write_read(address, wr_buffer, rd_buffer).await.map_err(I2cBusDeviceError::I2c)?;
            Ok(())
        }
    }

    type TransactionFuture<'a, 'b> = impl Future<Output = Result<(), Self::Error>> + 'a where Self: 'a, 'b: 'a;

    fn transaction<'a, 'b>(
        &'a mut self,
        address: u8,
        operations: &'a mut [embedded_hal_async::i2c::Operation<'b>],
    ) -> Self::TransactionFuture<'a, 'b> {
        let _ = address;
        let _ = operations;
        async move { todo!() }
    }
}