<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET Users List</name>
   <tag></tag>
   <elementGuidId>99e5c4d3-ba7e-4cee-b816-7b4fb40042b8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>b47f78eb-155f-4ddf-a3a3-b309d40156f5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>efa256f7-d1ab-43b2-a827-bc2c0a2d1453</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.endPoint}/api/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>7</defaultValue>
      <description></description>
      <id>ccd9f74d-2ae1-412b-b7e8-d9b69b3f985d</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>'michael.lawson@reqres.in'</defaultValue>
      <description></description>
      <id>f1b99e40-fc6a-40bc-83f2-233dcf274a10</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>'Michael'</defaultValue>
      <description></description>
      <id>cf403d9d-53f4-4cfe-92a5-551f68f40017</id>
      <masked>false</masked>
      <name>firstname</name>
   </variables>
   <variables>
      <defaultValue>'Lawson'</defaultValue>
      <description></description>
      <id>ff4934e0-4685-4071-8423-369dcb55273a</id>
      <masked>false</masked>
      <name>lastname</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()




WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'data[0].email', &quot;michael.lawson@reqres.in&quot;)
WS.verifyElementPropertyValue(response, 'data[0].avatar', &quot;https://reqres.in/img/faces/7-image.jpg&quot;)
WS.verifyElementPropertyValue(response, 'data[0].last_name', &quot;Lawson&quot;)
WS.verifyElementPropertyValue(response, 'data[1].first_name', &quot;Lindsay&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
