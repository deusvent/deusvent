//! Fixtures for testing

use aws_lambda_events::apigw::ApiGatewayWebsocketProxyRequest;
use lambda_runtime::{Context, LambdaEvent};

/// Returns API Gateway Websocket full serialized request which can be used for testing
pub fn event_with_body(body: String) -> LambdaEvent<ApiGatewayWebsocketProxyRequest> {
    let api_gateway_request = r#"{
    "resource": null,
    "path": null,
    "headers": {},
    "multiValueHeaders": {},
    "queryStringParameters": {},
    "multiValueQueryStringParameters": {},
    "pathParameters": {},
    "stageVariables": {},
    "requestContext": {
        "accountId": null,
        "resourceId": null,
        "stage": "v1",
        "requestId": "etACDFCcoAMFqDA=",
        "identity": {
            "cognitoIdentityPoolId": null,
            "accountId": null,
            "cognitoIdentityId": null,
            "caller": null,
            "apiKey": null,
            "apiKeyId": null,
            "accessKey": null,
            "sourceIp": "87.95.116.76",
            "cognitoAuthenticationType": null,
            "cognitoAuthenticationProvider": null,
            "userArn": null,
            "userAgent": null,
            "user": null
        },
        "resourcePath": null,
        "apiId": "o6t1ti5ko2",
        "connectedAt": 1727337381989,
        "connectionId": "etAB-deZIAMCK2g=",
        "domainName": "api.deusvent.com",
        "error": null,
        "eventType": "MESSAGE",
        "extendedRequestId": "etACDFCcoAMFqDA=",
        "integrationLatency": null,
        "messageDirection": "IN",
        "messageId": "etACDdeiIAMCK2g=",
        "requestTime": "26/Sep/2024:07:56:22 +0000",
        "requestTimeEpoch": 1727337382493,
        "routeKey": "TEST",
        "status": null,
        "disconnectStatusCode": null,
        "disconnectReason": null
    },
    "body": "",
    "isBase64Encoded": false
}"#;
    let mut request: ApiGatewayWebsocketProxyRequest =
        serde_json::from_str(api_gateway_request).expect("Fixture should be valid");
    request.body = Some(body);
    LambdaEvent::new(request, Context::default())
}
