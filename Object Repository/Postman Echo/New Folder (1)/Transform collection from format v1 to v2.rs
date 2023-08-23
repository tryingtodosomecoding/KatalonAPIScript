<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Transform collection from format v1 to v2</name>
   <tag></tag>
   <elementGuidId>cd60c57b-7a92-48a5-9ecf-1e300b356b59</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: \&quot;7875be4b-917d-4aff-8cc4-5606c36bf418\&quot;,\n  \&quot;name\&quot;: \&quot;Sample Postman Collection\&quot;,\n  \&quot;description\&quot;: \&quot;A sample collection to demonstrate collections as a set of related requests\&quot;,\n  \&quot;order\&quot;: [\n    \&quot;4d9134be-e8bf-4693-9cd7-1c0fc66ae739\&quot;,\n    \&quot;141ba274-cc50-4377-a59c-e080066f375e\&quot;,\n    \&quot;4511ca8b-0bc7-430f-b894-a7ec1036f322\&quot;\n  ],\n  \&quot;folders\&quot;: [],\n  \&quot;requests\&quot;: [\n    {\n      \&quot;id\&quot;: \&quot;4d9134be-e8bf-4693-9cd7-1c0fc66ae739\&quot;,\n      \&quot;name\&quot;: \&quot;A simple GET request\&quot;,\n      \&quot;collectionId\&quot;: \&quot;877b9dae-a50e-4152-9b89-870c37216f78\&quot;,\n      \&quot;method\&quot;: \&quot;GET\&quot;,\n      \&quot;headers\&quot;: \&quot;\&quot;,\n      \&quot;data\&quot;: [],\n      \&quot;rawModeData\&quot;: \&quot;\&quot;,\n      \&quot;tests\&quot;: \&quot;tests[\u0027response code is 200\u0027] \u003d (responseCode.code \u003d\u003d\u003d 200);\&quot;,\n      \&quot;preRequestScript\&quot;: \&quot;\&quot;,\n      \&quot;url\&quot;: \&quot;https://postman-echo.com/get?source\u003dnewman-sample-github-collection\&quot;\n    },\n    {\n      \&quot;id\&quot;: \&quot;141ba274-cc50-4377-a59c-e080066f375e\&quot;,\n      \&quot;name\&quot;: \&quot;A simple POST request\&quot;,\n      \&quot;collectionId\&quot;: \&quot;877b9dae-a50e-4152-9b89-870c37216f78\&quot;,\n      \&quot;method\&quot;: \&quot;POST\&quot;,\n      \&quot;headers\&quot;: \&quot;Content-Type: text/plain\&quot;,\n      \&quot;dataMode\&quot;: \&quot;raw\&quot;,\n      \&quot;data\&quot;: [],\n      \&quot;rawModeData\&quot;: \&quot;Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...\&quot;,\n      \&quot;url\&quot;: \&quot;https://postman-echo.com/post\&quot;\n    },\n    {\n      \&quot;id\&quot;: \&quot;4511ca8b-0bc7-430f-b894-a7ec1036f322\&quot;,\n      \&quot;name\&quot;: \&quot;A simple POST request with JSON body\&quot;,\n      \&quot;collectionId\&quot;: \&quot;877b9dae-a50e-4152-9b89-870c37216f78\&quot;,\n      \&quot;method\&quot;: \&quot;POST\&quot;,\n      \&quot;headers\&quot;: \&quot;Content-Type: application/json\&quot;,\n      \&quot;dataMode\&quot;: \&quot;raw\&quot;,\n      \&quot;data\&quot;: [],\n      \&quot;rawModeData\&quot;: \&quot;{\\\&quot;text\\\&quot;:\\\&quot;Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...\\\&quot;}\&quot;,\n      \&quot;url\&quot;: \&quot;https://postman-echo.com/post\&quot;\n    }\n  ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>5624619c-b2bc-453f-9553-0a30d2306275</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://postman-echo.com/transform/collection?from=1&amp;to=2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
