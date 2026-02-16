#![allow(clippy::useless_conversion)]  // PyO3 proc macro generates .into() on PyErr
// PyO3 bindings for the MTG RL environment.
//
// Exposes a Gymnasium-compatible Python API:
//
//   import mtg_python
//   env = mtg_python.MtgEnv()
//   obs = env.reset()
//   obs, reward, terminated, truncated, info = env.step(action)
//   mask = env.action_mask()
//
// Build with maturin: `maturin develop` or `maturin build --release`

use pyo3::prelude::*;
use pyo3::types::PyDict;

use mtg_ai::action_space::PHASE1_ACTION_SIZE;
use mtg_ai::gym::{GymConfig, MtgGymEnv};
use mtg_ai::observation::OBSERVATION_SIZE;

/// Gymnasium step return type: (observation, reward, terminated, truncated, info).
type StepResult<'py> = (Vec<f32>, f32, bool, bool, Bound<'py, PyDict>);

/// Python-facing MTG Gymnasium environment.
///
/// Usage from Python:
/// ```python
/// import mtg_python
/// env = mtg_python.MtgEnv(max_turns=100, seed=42)
/// obs = env.reset()
/// while True:
///     mask = env.action_mask()
///     action = select_action(obs, mask)  # your policy
///     obs, reward, terminated, truncated, info = env.step(action)
///     if terminated or truncated:
///         break
/// ```
#[pyclass]
struct MtgEnv {
    inner: MtgGymEnv,
}

#[pymethods]
impl MtgEnv {
    #[new]
    #[pyo3(signature = (max_turns=100, seed=None, intermediate_reward_scale=0.01))]
    fn new(
        max_turns: u32,
        seed: Option<u64>,
        intermediate_reward_scale: f32,
    ) -> Self {
        MtgEnv {
            inner: MtgGymEnv::new(GymConfig {
                max_turns,
                seed,
                intermediate_reward_scale,
            }),
        }
    }

    /// Reset the environment and return the initial observation as a list of floats.
    #[pyo3(signature = (seed=None))]
    fn reset(&mut self, seed: Option<u64>) -> Vec<f32> {
        let obs = self.inner.reset(seed);
        obs.features
    }

    /// Take a step in the environment.
    ///
    /// Args:
    ///     action: Integer action index (0..action_space_size).
    ///
    /// Returns:
    ///     Tuple of (observation, reward, terminated, truncated, info_dict).
    fn step<'py>(
        &mut self,
        py: Python<'py>,
        action: usize,
    ) -> PyResult<StepResult<'py>> {
        let result = self.inner.step(action);
        let info = PyDict::new_bound(py);
        for (k, v) in &result.info {
            info.set_item(k, v)?;
        }
        Ok((
            result.observation.features,
            result.reward,
            result.terminated,
            result.truncated,
            info,
        ))
    }

    /// Get the current legal action mask as a list of booleans.
    fn action_mask(&self) -> Vec<bool> {
        self.inner.action_mask().mask
    }

    /// Get the observation space size (length of observation vector).
    fn observation_space_size(&self) -> usize {
        self.inner.observation_space_size()
    }

    /// Get the action space size (number of possible actions).
    fn action_space_size(&self) -> usize {
        self.inner.action_space_size()
    }
}

/// The mtg_python Python module.
#[pymodule]
fn mtg_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", "0.1.0")?;
    m.add("OBSERVATION_SIZE", OBSERVATION_SIZE)?;
    m.add("ACTION_SPACE_SIZE", PHASE1_ACTION_SIZE)?;
    m.add_class::<MtgEnv>()?;
    Ok(())
}
