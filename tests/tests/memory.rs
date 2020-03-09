use heim::units::information::byte;
use tests::prelude::*;

/// bytes tolerance for system-wide memory related tests
const MEMORY_TOLERANCE: u64 = 500 * 1024;

#[tokio::test]
async fn memory() -> tests::Result<()> {
    let memory = heim::memory::memory().await?;

    cfg_if::cfg_if! {
        if #[cfg(target_os = "linux")] {
            use heim::memory::os::linux::MemoryExt;

            let (total, used, free, shared) = tests::linux::free()?;

            assert_delta_le!(memory.total().get::<byte>(), total, MEMORY_TOLERANCE);
            assert_delta_le!(memory.used().get::<byte>(), used, MEMORY_TOLERANCE);
            assert_delta_le!(memory.shared().get::<byte>(), shared, MEMORY_TOLERANCE);
            assert_delta_le!(memory.free().get::<byte>(), free, MEMORY_TOLERANCE);

        }
    }

    Ok(())
}
