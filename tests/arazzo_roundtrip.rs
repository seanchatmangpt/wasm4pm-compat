use wasm4pm_compat::arazzo::{ArazzoDescription, AsyncAction, SourceDescriptionType};

#[test]
fn parses_arazzo_1_1_async_workflow_shape() {
    let src = r#"
{
  "arazzo": "1.1.0",
  "$self": "https://example.test/workflows.arazzo.json",
  "info": {
    "title": "Inter-engine dispatch",
    "version": "26.7.10"
  },
  "sourceDescriptions": [
    {
      "name": "remoteEngine",
      "url": "https://example.test/openapi.json",
      "type": "openapi"
    },
    {
      "name": "remoteEvents",
      "url": "https://example.test/asyncapi.json",
      "type": "asyncapi"
    }
  ],
  "workflows": [
    {
      "workflowId": "dispatchRemoteWorkflow",
      "steps": [
        {
          "stepId": "dispatch",
          "operationId": "$sourceDescriptions.remoteEngine.dispatchWorkflow",
          "successCriteria": [
            { "condition": "$statusCode == 202" }
          ]
        },
        {
          "stepId": "receiveResult",
          "operationPath": "$sourceDescriptions.remoteEvents.workflowResult",
          "action": "receive",
          "correlationId": "$inputs.correlationId",
          "timeout": 6000,
          "dependsOn": ["dispatch"]
        }
      ]
    }
  ]
}
"#;

    let doc: ArazzoDescription = serde_json::from_str(src).unwrap();
    assert_eq!(doc.arazzo, "1.1.0");
    assert_eq!(
        doc.source_descriptions[0].source_type,
        Some(SourceDescriptionType::Openapi)
    );
    assert_eq!(
        doc.workflows[0].steps[1].action,
        Some(AsyncAction::Receive)
    );
    assert_eq!(
        doc.workflows[0].steps[1].depends_on,
        vec!["dispatch".to_owned()]
    );

    let replay = serde_json::to_string(&doc).unwrap();
    let reparsed: ArazzoDescription = serde_json::from_str(&replay).unwrap();
    assert_eq!(doc, reparsed);
}
