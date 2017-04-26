use std::collections::BTreeMap;

{{#each definitions~}}

/// {{this.description}}
#[derive(Serialize, Deserialize, Debug)]
pub struct {{@key}} {
  {{#each this.properties}}pub {{@key}}: {{datatype this.type this.format}}{{/each}}
}

{{/each~}}
