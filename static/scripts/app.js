var socket = null;
function connect() {
  disconnect()
  const { location } = window

  ws_scheme = window.location.protocol == "https:" ? "wss" : "ws";
  wsUri = ws_scheme + '://' + window.location.host + "/ws";
  //wsUri = ws_scheme + "://89.108.110.98:8082/ws";

  socket = new WebSocket(wsUri)

  socket.onopen = () => {
    console.log('Connected')
  }

  socket.onmessage = (ev) => {
    json_data = JSON.parse(ev.data)
    // обновляем статистику страницы - навый пользователь смотрит
    if (json_data["types"] == "page_view" && document.body.querySelector(".doc_title").getAttribute("page-id") == json_data["id"]) {
      document.body.querySelector(".real_wiew").innerHTML = json_data["data"];
      //console.log('Смотрит страницу: ' + json_data["id"]);
    }
    // обновляем статистику страницы - навый пользователь ушел
    else if (json_data["types"] == "end_page_view" && document.body.querySelector(".doc_title").getAttribute("page-id") == json_data["id"]) {
      real_wiew = document.body.querySelector(".real_wiew");
      document.body.querySelector(".real_wiew").innerHTML = json_data["data"];
      //console.log('Ушел со страницы: ' + json_data["id"]);
    }
    // обновляем статистику объекта - навый пользователь смотрит
    else if (json_data["types"] == "object_view" && document.body.querySelector(".doc_title").getAttribute("data-id") == json_data["id"]) {
      document.body.querySelector(".real_wiew").innerHTML = json_data["data"];
      //console.log('Смотрит объект: ' + json_data["id"]);
    }
    // обновляем статистику объекта - навый пользователь ушел
    else if (json_data["types"] == "end_object_view" && document.body.querySelector(".doc_title").getAttribute("data-id") == json_data["id"]) {
      real_wiew = document.body.querySelector(".real_wiew");
      document.body.querySelector(".real_wiew").innerHTML = json_data["data"];
      //console.log('Ушел с объекта: ' + json_data["id"]);
    }
  }

  socket.onclose = () => {
    //console.log('Disconnected')
    socket = null
  }
}

function disconnect() {
  if (socket) {
    //console.log('Disconnecting...')
    socket.close()
    socket = null
  }
}

//пока сокеты оставим неактивными
//connect() 

function check_first_load() {
    span = document.body.querySelector("#reload");
    loc = window.location.href;
    if (loc.indexOf('template') > -1) {
      url = loc + "&ajax=1"; 
    } 
    else {
      url = loc + "?ajax=1"; 
    }
  
      ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'GET', url, true );
      ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
      ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
            //get_custom_design();
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            span.innerHTML = elem_.innerHTML;
            //get_or_create_cookie_user();
            //get_active_button();
            //get_page_view_time(120); 
            scrolled(document.body.querySelector(".span"));
            window.history.pushState ({"url":loc}, document.title, loc);
            //create_desing_menu(); 
        }
      }
      ajax_link.send();
}
check_first_load();