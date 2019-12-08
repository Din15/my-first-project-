# my-first-project-
automated test
import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager
import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable
RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
String requestMethod = "GET"
String endpoint = "https://naimi.kz/"
String authHeader = ""
TestObjectProperty header_ContentType = new TestObjectProperty(Content-Type, ConditionType.EQUALS, "text/html; charset=utf-8")
TestObjectProperty header_Accept = new TestObjectProperty("Accept", ConditionType.EQUALS, "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,/;q=0.8")
TestObjectProperty header_Authorization = new TestObjectProperty("Authorization", ConditionType.EQUALS, authHeader)
not_run: TestObjectProperty header_Cookies = new TestObjectProperty("Cookie", ConditionType.EQUALS, cookies)
ArrayList defaultHeader = Arrays.asList(header_ContentType, header_Accept)
RequestObject ro = new RequestObject()
ro.setRestUrl(endpoint)
ro.setHttpHeaderProperties(defaultHeader)
ro.setRestRequestMethod(requestMethod)
ResponseObject resp = WS.sendRequest(ro)
println(resp.getStatusCode())
/* ответ 
<!DOCTYPE html>
<html lang="ru">
  <head>
    <!-- Google Tag Manager -->
    <script>(function (w, d, s, l, i) {
        w[l] = w[l] || [];
        w[l].push({
          'gtm.start':
          new Date().getTime(), event: 'gtm.js'
        }
                 );
        var f = d.getElementsByTagName(s)[0],
            j = d.createElement(s), dl = l != 'dataLayer' ? '&l=' + l : '';
        j.async = true;
        j.src =
          'https://www.googletagmanager.com/gtm.js?id=' + i + dl;
        f.parentNode.insertBefore(j, f);
      }
            )(window, document, 'script', 'dataLayer', 'GTM-NJH48QQ');
    </script>
    <!-- End Google Tag Manager -->
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1.0, user-scalable=no">
    <title>Naimi.kz - бесплатный сервис для поиска специалистов
    </title>
    <meta name="description" content=""/>
    <meta name="keywords" content="">
    <meta property="og:type" content="website"/>
    <meta property="og:site_name" content="Naimi.kz"/>
    <meta property="og:title"
          content="Naimi.kz - бесплатный сервис для поиска специалистов"/>
    <meta property="og:description"
          content="Naimi.kz - это бесплатная площадка для поиска специалистов. От сантехника до репетитора, от водителя до продавца. Лучшие специалисты Казахстана с анкетами и отзывами готовы приступить к работе прямо сейчас."/>
    <meta property="og:url" content="https://naimi.kz"/>
    <meta property="og:image" content="https://static.naimi.kz/j2yoGZ3TxX/img/sharing_logo.jpg?1574852165"/>
    <meta property="og:image:width" content="520"/>
    <meta property="og:image:height" content="200"/>
    <meta name="apple-itunes-app" content="app-id=1028949260">
    <meta name="google-play-app" content="app-id=kz.naimi.app">
    <link rel="canonical" href="">
    <link rel="next" href="">
    <link rel="prev" href="">
    <link rel="apple-touch-icon" sizes="57x57" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/apple-touch-icon-57x57.png?1574852165">
    <link rel="apple-touch-icon" sizes="60x60" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/apple-touch-icon-60x60.png?1574852165">
    <link rel="apple-touch-icon" sizes="72x72" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/apple-touch-icon-72x72.png?1574852165">
    <link rel="apple-touch-icon" sizes="76x76" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/apple-touch-icon-76x76.png?1574852165">
    <link rel="apple-touch-icon" sizes="114x114" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/apple-touch-icon-114x114.png?1574852165">
    <link rel="apple-touch-icon" sizes="120x120" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/apple-touch-icon-120x120.png?1574852165">
    <link rel="apple-touch-icon" sizes="144x144" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/apple-touch-icon-144x144.png?1574852165">
    <link rel="apple-touch-icon" sizes="152x152" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/apple-touch-icon-152x152.png?1574852165">
    <link rel="apple-touch-icon" sizes="180x180" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/apple-touch-icon-180x180.png?1574852165">
    <link rel="icon" type="image/png" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/favicon-32x32.png?1574852165" sizes="32x32">
    <link rel="icon" type="image/png" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/android-chrome-36x36.png?1574852165" sizes="36x36">
    <link rel="icon" type="image/png" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/android-chrome-48x48.png?1574852165" sizes="48x48">
    <link rel="icon" type="image/png" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/android-chrome-72x72.png?1574852165" sizes="72x72">
    <link rel="icon" type="image/png" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/android-chrome-96x96.png?1574852165" sizes="96x96">
    <link rel="icon" type="image/png" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/android-chrome-144x144.png?1574852165" sizes="144x144">
    <link rel="icon" type="image/png" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/android-chrome-192x192.png?1574852165" sizes="192x192">
    <link rel="icon" type="image/png" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/favicon-96x96.png?1574852165" sizes="96x96">
    <link rel="icon" type="image/png" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/favicon-16x16.png?1574852165" sizes="16x16">
    <link rel="manifest" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/manifest.json?1574852165">
    <link rel="shortcut icon" href="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/favicon.ico?1574852165">
    <meta name="msapplication-TileColor" content="#00aba9">
    <meta name="msapplication-TileImage" content="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/mstile-144x144.png?1574852165">
    <meta name="msapplication-config" content="https://static.naimi.kz/j2yoGZ3TxX/img/favicons/browserconfig.xml?1574852165">
    <meta name="theme-color" content="#ffffff">
    <script>
    </script>
    <link rel="stylesheet" href="https://unpkg.com/purecss@1.0.0/build/pure-min.css"
          integrity="sha384-nn4HPE8lTHyVtfCBi5yW9d20FjT8BJwUXyWZT9InLYax14RDjBj46LmSztkmNP9w" crossorigin="anonymous">
    <link href="https://fonts.googleapis.com/css?family=Roboto:400,500,700&amp;subset=cyrillic,cyrillic-ext,latin-ext"
          rel="stylesheet">
    <link href="https://fonts.googleapis.com/icon?family=Material+Icons" rel="stylesheet">
    <link href="https://static.naimi.kz/j2yoGZ3TxX/css/app.css?id=4b6e10e9ccffacce2684" */
