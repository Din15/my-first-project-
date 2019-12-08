<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>New Request (1)</name>
   <tag></tag>
   <elementGuidId>033072b0-ed05-46c7-9397-b381e4568efa</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://naimi.kz/?String RequestMethod=Get</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
String requestMethod = &quot;GET&quot;
String endpoint = &quot;https://naimi.kz/&quot;
String authHeader = &quot;&quot;

TestObjectProperty header_ContentType = new TestObjectProperty(Content-Type, ConditionType.EQUALS, &quot;text/html; charset=utf-8&quot;)

TestObjectProperty header_Accept = new TestObjectProperty(&quot;Accept&quot;, ConditionType.EQUALS, &quot;text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,/;q=0.8&quot;)

TestObjectProperty header_Authorization = new TestObjectProperty(&quot;Authorization&quot;, ConditionType.EQUALS, authHeader)

not_run: TestObjectProperty header_Cookies = new TestObjectProperty(&quot;Cookie&quot;, ConditionType.EQUALS, cookies)

ArrayList defaultHeader = Arrays.asList(header_ContentType, header_Accept)

RequestObject ro = new RequestObject()
ro.setRestUrl(endpoint)
ro.setHttpHeaderProperties(defaultHeader)
ro.setRestRequestMethod(requestMethod)

ResponseObject resp = WS.sendRequest(ro)
println(resp.getStatusCode())</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
