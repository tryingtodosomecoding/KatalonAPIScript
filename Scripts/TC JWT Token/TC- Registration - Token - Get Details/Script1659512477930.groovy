import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import org.assertj.core.api.Assertions as Assertions
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import groovy.json.JsonSlurper as JsonSlurper
import internal.GlobalVariable as GlobalVariable

'Create the body for the request'
def stringBody = '{  "password": "Guns and Bikes",  "username": "John Wick"}'

'Send the request for registration'
ResponseObject Response = WS.sendRequest(findTestObject('OR JWT Token/PostRequest - JWT', [('url') : GlobalVariable.Reg_url, ('body') : stringBody]))

'Assert the status code'
Assertions.assertThat(Response.getStatusCode()).isEqualTo(200)

println(Response.getResponseBodyContent())

'Send the request for auth and capture the response'
postResponse = WS.sendRequest(findTestObject('OR JWT Token/PostRequest - JWT', [('url') : GlobalVariable.auth_url, ('body') : stringBody]))

println(postResponse.getResponseBodyContent())

'Assert the status code'
Assertions.assertThat(postResponse.getStatusCode()).isEqualTo(200)

'Extract the token'
JsonSlurper parser = new JsonSlurper()

def afterParsing = parser.parseText(postResponse.getResponseBodyContent())

'Print the token at console'
println(afterParsing.token)

'Send the request with jwt token'
ResponseObject getResponse = WS.sendRequest(findTestObject('Object Repository/OR JWT Token/GetRequest - Jwt', [('jwt_token') : afterParsing.token]))

'Assert on the status code'
Assertions.assertThat(getResponse.getStatusCode()).isEqualTo(200)

