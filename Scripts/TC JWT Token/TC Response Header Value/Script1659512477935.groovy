import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

ResponseObject response = WS.sendRequest(findTestObject('OR JWT Token/PostRequest - JWT with variable'))

println(response.getResponseBodyContent())

println(response.getHeaderFields().get('Server').toString())

String textProtection = response.getHeaderFields().get('X-Xss-Protection').toString()

println("hello 1:- "+textProtection)

String newvar = CustomKeywords.'customeKeyword.ApplicationFunction.extractText'(textProtection)

println("hello 2:- "+newvar)

println(response.getHeaderFields().get('Strict-Transport-Security').toString())

println(response.getHeaderFields().get('X-Frame-Options').toString())

println(response.getHeaderFields().get('Pragma').toString())

println(response.getHeaderFields().get('X-Content-Type-Options').toString())



