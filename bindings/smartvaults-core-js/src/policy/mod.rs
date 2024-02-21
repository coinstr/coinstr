// Copyright (c) 2022-2024 Smart Vaults
// Distributed under the MIT software license

use core::ops::Deref;

use smartvaults_core::policy::Policy;
use wasm_bindgen::prelude::*;

pub mod template;

use self::template::JsPolicyTemplate;
use crate::error::{into_err, Result};
use crate::network::JsNetwork;

#[wasm_bindgen(js_name = Policy)]
pub struct JsPolicy {
    inner: Policy,
}

impl From<Policy> for JsPolicy {
    fn from(inner: Policy) -> Self {
        Self { inner }
    }
}

impl Deref for JsPolicy {
    type Target = Policy;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<JsPolicy> for Policy {
    fn from(policy: JsPolicy) -> Self {
        policy.inner
    }
}

#[wasm_bindgen(js_class = Policy)]
impl JsPolicy {
    /// Construct `Policy` from descriptor string
    ///
    /// The descriptor must be typed, for example: `tr(...)`
    #[wasm_bindgen(js_name = fromDescriptor)]
    pub fn from_descriptor(
        name: &str,
        description: &str,
        descriptor: &str,
        network: JsNetwork,
    ) -> Result<JsPolicy> {
        Ok(Self {
            inner: Policy::from_descriptor(name, description, descriptor, network.into())
                .map_err(into_err)?,
        })
    }

    /// Construct `Policy` from miniscript
    ///
    /// <https://bitcoin.sipa.be/miniscript/>
    #[wasm_bindgen(js_name = fromMiniscript)]
    pub fn from_miniscript(
        name: &str,
        description: &str,
        policy: &str,
        network: JsNetwork,
    ) -> Result<JsPolicy> {
        Ok(Self {
            inner: Policy::from_policy(name, description, policy, network.into())
                .map_err(into_err)?,
        })
    }

    /// Try to construct `Policy` from descriptor string or miniscript policy
    ///
    /// Internally try before to construct the `Policy` from a descriptor string. If fail, try from miniscript policy.
    #[wasm_bindgen(js_name = fromDescOrMiniscript)]
    pub fn from_desc_or_miniscript(
        name: &str,
        description: &str,
        desc_or_miniscript: &str,
        network: JsNetwork,
    ) -> Result<JsPolicy> {
        Ok(Self {
            inner: Policy::from_desc_or_policy(
                name,
                description,
                desc_or_miniscript,
                network.into(),
            )
            .map_err(into_err)?,
        })
    }

    /// Construct `Policy` from `PolicyTemplate`
    #[wasm_bindgen(js_name = fromTemplate)]
    pub fn from_template(
        name: &str,
        description: &str,
        template: &JsPolicyTemplate,
        network: JsNetwork,
    ) -> Result<JsPolicy> {
        Ok(Self {
            inner: Policy::from_template(
                name,
                description,
                template.deref().clone(),
                network.into(),
            )
            .map_err(into_err)?,
        })
    }

    pub fn descriptor(&self) -> String {
        self.inner.descriptor().to_string()
    }

    /// Get network
    pub fn network(&self) -> JsNetwork {
        self.inner.network().into()
    }

    /// Check if `Policy` has an `absolute` or `relative` timelock
    #[wasm_bindgen(js_name = hasTimelock)]
    pub fn has_timelock(&self) -> bool {
        self.inner.has_timelock()
    }

    /// Check if `Policy` has a `absolute` timelock
    #[wasm_bindgen(js_name = hasAbsoluteTimelock)]
    pub fn has_absolute_timelock(&self) -> bool {
        self.inner.has_absolute_timelock()
    }

    /// Check if `Policy` has a `relative` timelock
    #[wasm_bindgen(js_name = hasRelativeTimelock)]
    pub fn has_relative_timelock(&self) -> bool {
        self.inner.has_relative_timelock()
    }
}
