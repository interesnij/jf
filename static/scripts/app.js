function on(elSelector, eventName, selector, fn) {var element = document.querySelector(elSelector);element.addEventListener(eventName, function(event) {var possibleTargets = element.querySelectorAll(selector);var target = event.target;for (var i = 0, l = possibleTargets.length; i < l; i++) {var el = target;var p = possibleTargets[i];while (el && el !== element) {if (el === p) {return fn.call(p, event);}el = el.parentNode;}}});}

$height = parseFloat(window.innerHeight * 0.000264).toFixed(2);
  $seconds = 1;
  $user_id = 0;
  $page_time_end = false;
  
  $window_height = 0;
  $window_seconds = 1;
$window_time_end = false;

var delayedExec = function(after, fn) {
    var timer;
    return function() {
        timer && clearTimeout(timer);
        timer = setTimeout(fn, after);
    };
};

function scrolled(_block) {
    offset = 0;
    window.onscroll = function() {
      // программа отслеживает окончание прокрутки
      //scrollStopper();
      // программа считает секунды для внесения в стат страницы и списка, если он есть.
      if ($page_time_end) {
        get_page_view_time(120);
        $page_time_end = false;
      };
      if ($window_time_end) {
        get_window_view_timer(120);
        $window_time_end = false;
      };

      // программа останавливает отчет времени просмотра элементов, на которых остановился
      // пользователь, записывает его всем новым элементам pag, затем их добавляет в основной
      // список стата, обнуляет счетчик и очищает список новых элементов.
      if ((window.innerHeight + window.pageYOffset) > offset) {
        offset = window.innerHeight + window.pageYOffset;
        $height = parseFloat(offset * 0.000264).toFixed(2);
      };

      //try {
          box = _block.querySelector('.next_page_list');
          if (box && box.classList.contains("next_page_list")) {
              inViewport = elementInViewport(box);
              if (inViewport) {
                  box.classList.remove("next_page_list");
                  paginate(box);
              }
          };
      //} catch {return}
    }
};
function paginate(block) {
        var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
        link_3.open('GET', location.protocol + "//" + location.host + block.getAttribute("data-link") + "&ajax=2", true);
        link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

        link_3.onreadystatechange = function() {
            if (this.readyState == 4 && this.status == 200) {
                var elem = document.createElement('span');
                elem.innerHTML = link_3.responseText;
                block.parentElement.insertAdjacentHTML('beforeend', elem.querySelector(".is_paginate").innerHTML)
                block.remove()
            }
        }
        link_3.send();
};

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

function ajax_get_reload(url, history_enable, ajax) {
  var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', url + "?ajax=" + ajax, true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  ajax_link.onreadystatechange = function () {
    if ( this.readyState == 4 && this.status == 200 ) {
      rtr = document.getElementById('ajax');
      // статистика
      $link = document.location.pathname;
      meta_block = rtr.querySelector(".doc_title");
      $title = meta_block.getAttribute("data-title");
      //
      elem_ = document.createElement('span');
      elem_.innerHTML = ajax_link.responseText;
      sidebar = elem_.querySelector(".sidebar");

      rtr.innerHTML = elem_.innerHTML;

      try {
        _meta = rtr.querySelector(".doc_title");
        _title = _meta.getAttribute("data-title");
        _uri = "" + _meta.getAttribute("data-uri");
        _description = _meta.getAttribute("data-description");
        _image = "https://app.juslaw.com" + _meta.getAttribute("data-image");
        document.title = _title;
        document.querySelector('meta[name="url"]').setAttribute("content", _uri);
        document.querySelector('meta[name="title"]').setAttribute("content", _title);
        document.querySelector('meta[name="description"]').setAttribute("content", _description);
        document.querySelector('meta[name="image"]').setAttribute("content", _image);
        document.querySelector('link[rel="canonical"]').setAttribute("href", _uri);
      } catch { null };
      window.scrollTo(0,0);
      if (history_enable) { 
        window.history.pushState ({"url":url}, $title, url);
      }
      //get_active_button();
      //get_page_view_time(120);
      scrolled(rtr);
      //get_stat_meta($link, $title, $object_id, $page_id);
      get_document_opacity_1();
    }
  }
  ajax_link.send();
};

on('body', 'click', '.menu-button', function() {
  block = this.nextElementSibling;
  if (this.classList.contains("open")) {
    this.classList.remove("open");
    block.style.opacity = "1";
    block.style.visibility = "visible";
  }
  if (this.classList.contains("open")) {
    this.classList.add("open")
    block.style.opacity = "0";
    block.style.visibility = "hidden";
  }
});

on('body', 'click', '#logg', function() {
  _this = this;
  form = _this.parentElement.parentElement; 
  response1 = form.querySelector(".api_response1");
  response2 = form.querySelector(".api_response2");

  if (!form.querySelector("#id_email").value){
    form.querySelector("#id_email").style.border = "1px #FF0000 solid";
    response1.innerHTML = "Enter your email!";
    response1.classList.add("error");
    return
  }
  else if (!form.querySelector("#id_password").value){
    form.querySelector("#id_password").style.border = "1px #FF0000 solid";
    response2.innerHTML = "Enter the password!";
    response2.classList.add("error")
    return
  }
  else {
    _this.disabled = true;
  }

  form_data = new FormData(form);
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "https://backend.juslaw.com/api/v1/auth/login/", true );
  //link.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    //var jsonData = JSON.parse(link.response);
    localStorage.setItem("request_data", link.response)
    window.location.href = "/" 
  }

  else {
    _this.disabled = false;
    response.style.display = "block";
    //response.innerHTML = "Логин или пароль - неверный!";
    response.classList.add("error");
    form.querySelector("#id_email").style.display = "block";
    form.querySelector("#id_email").value = '';
    form.querySelector("#id_password").value = '';
  }};
  link.send(form_data);
});

on('body', 'click', '#signup', function() {
  _this = this;
  form = _this.parentElement;
  username = form.querySelector("#id_email");
  response1 = form.querySelector(".api_response1");
  response2 = form.querySelector(".api_response2");
  if (!username.value){
    username.style.border = "1px #FF0000 solid";
    toast_error("Enter your email!");
    return
  } else if (!form.querySelector("#id_password").value){
    form.querySelector("#id_password").style.border = "1px #FF0000 solid";
    toast_error("Enter your password!");
    return
  }
  else {
    this.disabled = true
  }

  form_data = new FormData(form);
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/signup/", true );

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    window.location.href = "/"
    }
  else {
    _this.disabled = false;
    response.style.display = "block";
    response.innerHTML = "not ok";
    response.classList.add("error");
  }};
  link.send(form_data);
});