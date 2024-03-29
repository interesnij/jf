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

function load_data(number, w_block) {
  console.log("=====================");
  console.log("start with #" + number);
  if (number == 0) {
    block = w_block.querySelector(".load_content");
    console.log("search load_content");
  }
  else {
    _class = ".load_content" + number;
    block = w_block.querySelector(_class);
    console.log("search load_content" + number);
  }

  if (!block && number == 0) {
    block = w_block.querySelector(".load_content1");
    number = number + 1;
    console.log("now search load_content1");
  }
  if (!block && number == 1) {
    block = w_block.querySelector(".load_content2");
    number = number + 1;
    console.log("now search load_content2");
  }
  if (block) {
    if (block.childNodes.length > 10) {
      console.log("block is not empty");
      console.log("childNodes.length", block.childNodes.length);
      console.log("innerHTML", block.innerHTML);
      return
    }
    else {
      console.log("block" + number + "no content!!");
      link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      link.open( 'GET', block.getAttribute("data-link"), true );
      if (localStorage.getItem('request_data') !== null) {
        link.setRequestHeader('Request-Data', localStorage.getItem('request_data'));
      }
      link.onreadystatechange = function () {
        if ( link.readyState == 4 && link.status == 200 ) {
            console.log("target block", block);
            block.innerHTML = link.responseText;
            new_number = number + 1;
            load_data(new_number, w_block);
        }
      };
      link.send();
    }
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
function get_file_data(_files) {
    file_data = new FormData();
    file_data.append("token", "111");
    file_data.append("folder", "111");
    file_data.append("object_id", "111");
    for (let i = 0; i < _files.length; i++) {
      file_data.append("file", _files[i]);
    }
    return file_data;
}


function validate(
    first_name, 
    last_name, 
    phone, 
    attachments, 
    country, 
    state,
    city,
    email,
    password1,
    password2,
    year,
    number,
  ) 
  {
    let is_error = false;

    if (first_name) {
        if (!first_name.value){
            first_name.style.border = "1px #FF0000 solid";
            first_name.nextElementSibling.innerHTML = "First Name is required";
            is_error = true;
        }
        else {
            first_name.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
            first_name.nextElementSibling.innerHTML = "";
        }
    }
    if (last_name) {
        if (!last_name.value){
            last_name.style.border = "1px #FF0000 solid";
            last_name.nextElementSibling.innerHTML = "Last Name is required";
            is_error = true;
        }
        else {
            last_name.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
            last_name.nextElementSibling.innerHTML = "";
        }
    }
    if (phone) {
        if (!phone.value){
            phone.style.border = "1px #FF0000 solid";
            phone.nextElementSibling.innerHTML = "Phone Name is required";
            is_error = true;
        }
        else {
            phone.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
            phone.nextElementSibling.innerHTML = "";
        }
    }
    if (attachments) {
          if (!attachments.value){
            parent = attachments.parentElement;
            parent.style.border = "1px #FF0000 solid";
            parent.querySelector(".attachments_error").innerHTML = "Attachments is required";
            is_error = true;
        }
        else {
            parent = attachments.parentElement;
            attachments.parent.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
            attachments.parent.querySelector(".attachments_error").innerHTML = "";
        }
    }
    if (country) {
        if (!country.value){
            country.style.border = "1px #FF0000 solid";
            country.nextElementSibling.nextElementSibling.innerHTML = "Country is required";
            is_error = true;
        }
        else {
            country.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
            country.nextElementSibling.nextElementSibling.innerHTML = "";
        }
    }
    if (state) {
        if (state != undefined && !state.value){
            state.style.border = "1px #FF0000 solid";
            state.nextElementSibling.nextElementSibling.innerHTML = "State is required";
            is_error = true;
        }
        else if (state != undefined) {
            state.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
            state.nextElementSibling.nextElementSibling.innerHTML = "";
        }
    }
    if (city) {
        if (!city.value){
            city.style.border = "1px #FF0000 solid";
            city.nextElementSibling.nextElementSibling.innerHTML = "City is required";
            is_error = true;
        }
        else {
            city.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
            city.nextElementSibling.nextElementSibling.innerHTML = "";
        } 
    }

    if (email) {
        if (!email.value){
            email.style.border = "1px #FF0000 solid";
            email.nextElementSibling.innerHTML = "Email is required";
            is_error = true;
        }
        else {
            email.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
            email.nextElementSibling.innerHTML = "";
        } 
    }
    if (password1) {
        if (!password1.value){
            password1.style.border = "1px #FF0000 solid";
            password1.nextElementSibling.innerHTML = "Password is required";
            is_error = true;
        }
        else {
            password1.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
            password1.nextElementSibling.innerHTML = "";
        } 
    }
    if (password2) {
        if (!password2.value){
            password2.style.border = "1px #FF0000 solid";
            password2.nextElementSibling.innerHTML = "Repeat the password";
            is_error = true;
        }
        else {
            password2.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
            password2.nextElementSibling.innerHTML = "";
        } 
    }
    if (year) {
        if (!year.value){
            year.style.border = "1px #FF0000 solid";
            year.nextElementSibling.innerHTML = "Year is required";
            is_error = true;
        }
        else {
            year.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
            year.nextElementSibling.innerHTML = "";
        } 
    }
    if (number) {
        if (!number.value){
            number.style.border = "1px #FF0000 solid";
            number.nextElementSibling.innerHTML = "Number is required";
            is_error = true;
        }
        else {
            number.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
            number.nextElementSibling.innerHTML = "";
        } 
    }

    console.log("validate error", is_error);
    if (is_error) {
      return false;
    }
    return true;

}
  
function ajax_get_reload(url, history_enable, ajax, _class) {
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
          if (!_class) {
            //console.log("default block!");
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
          }
          else {
            document.body.querySelector("." + _class).innerHTML = elem_.querySelector("." + _class).innerHTML;
            //console.log("class exist!", _class);
          }
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
        load_data(0, meta_block);
        scrolled(meta_block);
        get_document_opacity_1();
      }
    }
    ajax_link.send();
  };


function get_image_priview(ggg, img) {
    entrou = false;
    img.click();
    img.onchange = function() {
        if (!entrou) {
            imgPath = img.value;
            extn = imgPath.substring(imgPath.lastIndexOf(".") + 1).toLowerCase();
            if (extn == "gif" || extn == "png" || extn == "jpg" || extn == "jpeg") {
                if (typeof FileReader != "undefined") {
                    if (ggg) {}
                    ggg.innerHTML = "";
                    reader = new FileReader();
                    reader.onload = function(e) {
                        $img = document.createElement("img");
                        $img.src = e.target.result;
                        ggg.append($img)
                    };
                    reader.readAsDataURL(img.files[0])
                }
            } else {
                this.value = null
            }
        }
        entrou = true;
        setTimeout(function() {
            entrou = false
        }, 1000)
    }
};

function check_first_load() {
    span = document.body.querySelector("#reload");
    url = window.location.href;
    
    if (url == "https://app2.juslaw.com/" && localStorage.getItem('request_data') !== null) {
        loc = JSON.parse(localStorage.getItem('request_data'));
        data = localStorage.getItem('request_data');
        loc = JSON.parse(data);
        url = "https://app2.juslaw.com/" + loc.user_type + "/overview";
        console.log("url", url);
    }
  
      ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'GET', url + "?ajax=1", true );
      //ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
      if (localStorage.getItem('request_data') !== null) {
        data = localStorage.getItem('request_data');
        ajax_link.setRequestHeader('Request-Data', data);
        console.log("stringify data", JSON.stringify(data));
        console.log("parse data", JSON.parse(data));
        console.log("data", data);
      } 
      ajax_link.onreadystatechange = function () { 
        if ( this.readyState == 4 && this.status == 200 ) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            span.innerHTML = elem_.innerHTML;
            load_data(0, span);
            scrolled(span);
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