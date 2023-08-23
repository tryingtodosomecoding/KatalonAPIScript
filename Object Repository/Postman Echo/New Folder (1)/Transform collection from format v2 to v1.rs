<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Transform collection from format v2 to v1</name>
   <tag></tag>
   <elementGuidId>d819630c-6a8a-4b36-81a4-6b42ed5f7f77</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;info\&quot;: {\n    \&quot;name\&quot;: \&quot;Sample Postman Collection\&quot;,\n    \&quot;schema\&quot;: \&quot;https://schema.getpostman.com/json/collection/v2.0.0/collection.json\&quot;,\n    \&quot;description\&quot;: \&quot;A sample collection to demonstrate collections as a set of related requests\&quot;\n  },\n\n  \&quot;item\&quot;: [{\n    \&quot;name\&quot;: \&quot;A simple GET request\&quot;,\n    \&quot;event\&quot;: [{\n      \&quot;listen\&quot;: \&quot;test\&quot;,\n      \&quot;script\&quot;: {\n        \&quot;type\&quot;: \&quot;text/javascript\&quot;,\n        \&quot;exec\&quot;: [\&quot;tests[\u0027response code is 200\u0027] \u003d (responseCode.code \u003d\u003d\u003d 200);\&quot;]\n      }\n    }],\n    \&quot;request\&quot;: {\n      \&quot;url\&quot;: \&quot;https://postman-echo.com/get?source\u003dnewman-sample-github-collection\&quot;,\n      \&quot;method\&quot;: \&quot;GET\&quot;\n    }\n  }, {\n    \&quot;name\&quot;: \&quot;A simple POST request\&quot;,\n    \&quot;request\&quot;: {\n      \&quot;url\&quot;: \&quot;https://postman-echo.com/post\&quot;,\n      \&quot;method\&quot;: \&quot;POST\&quot;,\n      \&quot;header\&quot;: [{\n        \&quot;key\&quot;: \&quot;Content-Type\&quot;,\n        \&quot;value\&quot;: \&quot;text/plain\&quot;\n      }],\n      \&quot;body\&quot;: {\n        \&quot;mode\&quot;: \&quot;raw\&quot;,\n        \&quot;raw\&quot;: \&quot;Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...\&quot;\n      }\n    }\n  }, {\n    \&quot;name\&quot;: \&quot;A simple POST request with JSON body\&quot;,\n    \&quot;request\&quot;: {\n      \&quot;url\&quot;: \&quot;https://postman-echo.com/post\&quot;,\n      \&quot;method\&quot;: \&quot;POST\&quot;,\n      \&quot;header\&quot;: [{\n        \&quot;key\&quot;: \&quot;Content-Type\&quot;,\n        \&quot;value\&quot;: \&quot;application/json\&quot;\n      }],\n      \&quot;body\&quot;: {\n        \&quot;mode\&quot;: \&quot;raw\&quot;,\n        \&quot;raw\&quot;: \&quot;{\\\&quot;text\\\&quot;:\\\&quot;Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...\\\&quot;}\&quot;\n      }\n    }\n  }]\n}&quot;,
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
      <webElementGuid>9e221649-0cc9-4e46-8bd3-f6b1be8f5b5e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://postman-echo.com/transform/collection?from=2&amp;to=1</restUrl>
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
