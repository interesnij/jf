class ToastManager {
  constructor() {
      this.id = 0;
      this.toasts = [];
      this.icons = {
          'SUCCESS': "",
          'ERROR': '',
          'INFO': '',
          'WARNING': '',
      };
      var body = document.querySelector('body');
      this.toastsContainer = document.createElement('div');
      this.toastsContainer.classList.add('toasts', 'border-0');
      body.appendChild(this.toastsContainer)
  }
  showSuccess(message) {
      return this._showToast(message, 'SUCCESS')
  }
  showError(message) {
      return this._showToast(message, 'ERROR')
  }
  showInfo(message) {
      return this._showToast(message, 'INFO')
  }
  showWarning(message) {
      return this._showToast(message, 'WARNING')
  }
  _showToast(message, toastType) {
      var newId = this.id + 1;
      var newToast = document.createElement('div');
      newToast.style.display = 'inline-block';
      newToast.classList.add(toastType.toLowerCase());
      newToast.classList.add('toast');
      newToast.innerHTML = `<progress max="100"value="0"></progress><h3>${message}</h3>`;
      var newToastObject = {
          id: newId,
          message,
          type: toastType,
          timeout: 4000,
          progressElement: newToast.querySelector('progress'),
          counter: 0,
          timer: setInterval(() => {
              newToastObject.counter += 1000 / newToastObject.timeout;
              newToastObject.progressElement.value = newToastObject.counter.toString();
              if (newToastObject.counter >= 100) {
                  newToast.style.display = 'none';
                  clearInterval(newToastObject.timer);
                  this.toasts = this.toasts.filter((toast) => {
                      return toast.id === newToastObject.id
                  })
              }
          }, 10)
      }
      newToast.addEventListener('click', () => {
          newToast.style.display = 'none';
          clearInterval(newToastObject.timer);
          this.toasts = this.toasts.filter((toast) => {
              return toast.id === newToastObject.id
          })
      });
      this.toasts.push(newToastObject);
      this.toastsContainer.appendChild(newToast);
      return this.id++
  }
}

function toast_success(text) {
  var toasts = new ToastManager();
  toasts.showSuccess(text)
}

function toast_error(text) {
  var toasts = new ToastManager();
  toasts.showError(text)
}

function toast_info(text) {
  var toasts = new ToastManager();
  toasts.showInfo(text)
}

function toast_warning(text) {
  var toasts = new ToastManager();
  toasts.showWarning(text)
}

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

      try {
          box = _block.querySelector('.next_page_list');
          if (box && box.classList.contains("next_page_list")) {
              inViewport = elementInViewport(box);
              if (inViewport) {
                  box.classList.remove("next_page_list");
                  paginate(box);
              }
          };
      } catch {return}
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

function load_contentt(block) { 
    if (block.childNodes.length) {
      console.log("block", block);
      console.log("block with content!!");
    }
    else {
      console.log("block no content!!");
      link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      link.open( 'GET', block.getAttribute("data-link"), true );
      if (localStorage.getItem('request_data') !== null) {
        link.setRequestHeader('Request-Data', localStorage.getItem('request_data'));
      }
      link.onreadystatechange = function () {
        if ( link.readyState == 4 && link.status == 200 ) {
            console.log("target block", block);
            block.innerHTML = link.responseText;
        }
      };
      link.send();
    }
}

function load_data1() {
  block = document.body.querySelector(".load_content1");
  if (block) {
    if (block.childNodes.length) {
      console.log("block", block);
      console.log("block with content!!");
    }
    else {
      console.log("block no content!!");
      link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      link.open( 'GET', block.getAttribute("data-link"), true );
      if (localStorage.getItem('request_data') !== null) {
        link.setRequestHeader('Request-Data', localStorage.getItem('request_data'));
      }
      link.onreadystatechange = function () {
        if ( link.readyState == 4 && link.status == 200 ) {
            console.log("target block", block);
            block.innerHTML = link.responseText;
        }
      };
      link.send();
    }
  }
}
function load_data2() {
  block = document.body.querySelector(".load_content2");
  if (block) {
    if (block.childNodes.length) {
      console.log("block", block);
      console.log("block with content!!");
    }
    else {
      console.log("block no content!!");
      link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      link.open( 'GET', block.getAttribute("data-link"), true );
      if (localStorage.getItem('request_data') !== null) {
        link.setRequestHeader('Request-Data', localStorage.getItem('request_data'));
      }
      link.onreadystatechange = function () {
        if ( link.readyState == 4 && link.status == 200 ) {
            console.log("target block", block);
            block.innerHTML = link.responseText;
        }
      };
      link.send();
    }
  }
}

function load_data1111() {
  blocks = document.body.querySelectorAll(".load_content");
  for (let i = 0; i < blocks.length; i++) { 
    load_contentt(blocks[i])
  } 
}

function send_files(file_field) {
    file_data = new FormData();
    file_data.append("token", "111");
    file_data.append("folder", "111");
    file_data.append("object_id", "111");
    files = file_field.files;
    for (let i = 0; i < files.length; i++) {
        console.log("upload", files[i]);
        file_data.append("file", files[i]);
    }
    _link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    _link.open( 'POST', "https://k.juslaw.online/classic_create/", true );
    _link.onreadystatechange = function () {
    if ( _link.readyState == 4 && _link.status == 200 ) {
        data = JSON.parse(_link.responseText);
        res_files = data["files"];
        if (res_files.length < 1) {
            console.log("res_files.length < 1");
            return 0;
        }
        else {
            return res_files;
        }
    } else {
        console.log("return");
        return 0;
    }};
    _link.send(file_data);
}

function get_document_opacity_0() {
    document.body.style.overflowY = "hidden";
    //document.body.style.marginRight = "20px";
    overlay = document.body.querySelector(".body_overlay");
    overlay.style.visibility = "unset";
    overlay.style.opacity = "1";
  };
  function get_document_opacity_1() {
    document.body.style.overflowY = "scroll";
    //document.body.style.marginRight = "0";
    overlay = document.body.querySelector(".body_overlay");
    overlay.style.visibility = "hidden";
    overlay.style.opacity = "0";
};


function get_active_button() {
  //try {
    links = document.body.querySelectorAll(".navbar__item");
    path = document.location.pathname;
    console.log("path", path);
    for (var i = 0; i < links.length; i++){links[i].classList.remove("navbar__item--active")};

    if (path == "attorney/overview") {
      document.body.querySelector(".overview").classList.add("navbar__item--active");
    }
    else if (path == "attorney/matter") {
      document.body.querySelector(".matters").classList.add("navbar__item--active");
    }
    else if (path == "attorney/leads") {
      document.body.querySelector(".leads").classList.add("navbar__item--active");
    }
    else if (path == "attorney/documents") {
      document.body.querySelector(".documents").classList.add("navbar__item--active");
    }
    else if (path == "attorney/chat") {
      document.body.querySelector(".chats").classList.add("navbar__item--active");
    }
    else if (path == "attorney/billing") {
      document.body.querySelector(".billing").classList.add("navbar__item--active");
    }
    else if (path == "attorney/invoices") {
      document.body.querySelector(".invoices").classList.add("navbar__item--active");
    }
    else if (path == "attorney/bank") {
      document.body.querySelector(".bank").classList.add("navbar__item--active");
    }
    else if (path == "attorney/engagement") {
      document.body.querySelector(".engagement").classList.add("navbar__item--active");
    }
    else if (path == "attorney/news") {
      document.body.querySelector(".news").classList.add("navbar__item--active");
    }
    else if (path == "attorney/forums") {
      document.body.querySelector(".forums").classList.add("navbar__item--active");
    }
    else if (path == "attorney/contacts") {
      document.body.querySelector(".contacts").classList.add("navbar__item--active");
    }

    else if (path == "/client/overview") {
      document.body.querySelector(".overview").classList.add("navbar__item--active");
    }
    else if (path == "client/chat") {
      document.body.querySelector(".engagement").classList.add("navbar__item--active");
    }
    else if (path == "client/find") {
      document.body.querySelector(".find").classList.add("navbar__item--active");
    }
    else if (path == "client/forums") {
      document.body.querySelector(".forums").classList.add("navbar__item--active");
    }
    else if (path == "client/news") {
      document.body.querySelector(".news").classList.add("navbar__item--active");
    } 
  //} catch { null }
};
  
  function ajax_get_reload(url, history_enable, ajax) {
    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    ajax_link.open( 'GET', url + "?ajax=" + ajax, true );  
  
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    if (localStorage.getItem('request_data') !== null) {
      ajax_link.setRequestHeader('Request-Data', localStorage.getItem('request_data'));
    }
  
    ajax_link.onreadystatechange = function () {
      if ( this.readyState == 4 && this.status == 200 ) {
        meta_block = document.body.querySelector('.doc_title');
        // статистика
        
        try { 
          $link = document.location.pathname;
          meta_block = document.body.querySelector(".doc_title");
          $title = meta_block.getAttribute("data-title");
          //
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
  
          meta_block.innerHTML = elem_.innerHTML;
          _meta = elem_.querySelector(".doc_title");
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

        try {
            document.body.querySelector(".header__title").innerHTML = _title;
        } 
        catch { null }
  
        window.scrollTo(0,0);
        if (history_enable) { 
          window.history.pushState ({"url":url}, $title, url);
        }
        //get_active_button();
        load_data();
        scrolled(meta_block);
        get_document_opacity_1();
      }
    }
    ajax_link.send();
  };

  function check_first_load() {

    span = document.body.querySelector("#reload");
    url = window.location.href;
    
    if (url == "https://app2.juslaw.com/" && localStorage.getItem('request_data') !== null) {
        loc = JSON.parse(localStorage.getItem('request_data'));
        url = "https://app2.juslaw.com/" + loc.user_type + "/overview";
        console.log("url", url);
    }
  
      ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'GET', url + "?ajax=1", true );
      ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
      if (localStorage.getItem('request_data') !== null) {
        ajax_link.setRequestHeader('Request-Data', localStorage.getItem('request_data'));
      }
      ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            //console.log("elem_", elem_);
            //console.log("span", span);
            span.innerHTML = elem_.innerHTML;
            //get_active_button();
            load_data1();
            load_data2();
            scrolled(document.body.querySelector(".span"));
            window.history.pushState ({"url":url}, document.title, url);

            try {
              document.body.querySelector(".header__title").innerHTML = _title;
            } 
            catch { null }
            
        }
      }
      ajax_link.send();
}

check_first_load();