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

function elementInViewport(el){var bounds = el.getBoundingClientRect();return ((bounds.top + bounds.height > 0) && (window.innerHeight - bounds.top > 0));}

function close_fullscreen() {
    container = document.body.querySelector("#fullscreens_container");
    if (!container.innerHTML) {
      get_document_opacity_1();
      return
    };
    container = document.body.querySelector("#fullscreens_container");
    _window = container.querySelector(".card_fullscreen");
  
    //if (_window.querySelector(".doc_title")) {
    //  meta_block = _window.querySelector(".doc_title");
    //  $link = meta_block.getAttribute("data-link");
    //  $title = meta_block.getAttribute("data-title");
    //  $object_id = meta_block.getAttribute("object-id");
    //  $page_id = document.body.querySelector(".doc_title").getAttribute("page-id");
    //  get_window_stat_meta($link, $title, $object_id, $page_id);
    //}
  
    _window.remove();
  
    if (!container.innerHTML) {
      get_document_opacity_1();
    } else {
      prev_window = container.querySelector(".card_fullscreen");
      prev_window.classList.remove("hide");
    };
  };
  
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

function create_fullscreen(url, type_class, price) {
  container = document.body.querySelector("#fullscreens_container");

  try {
    count_items = container.querySelectorAll(".card_fullscreen").length + 1
  } catch {count_items = 0};

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link.open('GET', url, true);
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
        if (container.innerHTML) {
          prev_window = container.querySelector(".card_fullscreen");
          prev_window.classList.add("hide");
        };

        $parent_div = document.createElement("div");
        $parent_div.classList.add("card_fullscreen", "mb-30", "border", type_class);
        $parent_div.style.zIndex = 100 + count_items;
        $parent_div.style.opacity = "0";
        window_time_end = false;
        $window_height = 0;

        $hide_span = document.createElement("span");
        $hide_span.classList.add("this_fullscreen_hide");
        $loader = document.createElement("div");

        $loader.setAttribute("id", "fullscreen_loader");
        $hide_span.innerHTML = '<svg class="svg_default" style="position:fixed;" width="30" height="30" fill="currentColor" viewBox="0 0 24 24"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/><path d="M0 0h24v24H0z" fill="none"/></svg>';
        $parent_div.append($hide_span);
        $parent_div.append($loader);
        container.prepend($parent_div);

          elem = link.responseText;

          $loader.innerHTML = elem;
          height = $loader.scrollHeight*1 + 30;
          if (!price && height < 500 && !$loader.querySelector(".data_display")) {
            $parent_div.style.height = height + "px";
            $loader.style.overflowY = "unset";

            _height = (window.innerHeight - height - 50) / 2;
            $parent_div.style.top = _height + "px";
            prev_next_height = _height*1 + 50 + "px";
          } else {
            $parent_div.style.height = "100%";
            $parent_div.style.top = "15px";
            $loader.style.overflowY = "auto";
          };
          $parent_div.style.opacity = "1";
          if ($loader.querySelector(".data_display")) {
            $loader.style.overflowY = "unset";
          }

          get_document_opacity_0();
          //get_window_view_timer(120);
          offset = 0;
          $window_seconds = 1;

          $loader.onscroll = function() {
            //if ($window_time_end) {
            //  get_window_view_timer(120);
            //  $window_time_end = false;
            //};
            if ($loader.scrollHeight > offset) {
              offset = $loader.scrollHeight;
              $window_height = parseFloat(offset * 0.000264).toFixed(2);
            }
            if ($loader.querySelector(".next_page_list")) {
              box = $loader.querySelector('.next_page_list');
              if (box && box.classList.contains("next_page_list")) {
                  inViewport = elementInViewport(box);
                  if (inViewport) {
                      box.remove();
                      var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
                      link_3.open('GET', location.protocol + "//" + location.host + box.getAttribute("data-link") + "&ajax=2", true);
                      link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

                      link_3.onreadystatechange = function() {
                          if (this.readyState == 4 && this.status == 200) {
                              var elem = document.createElement('span');
                              elem.innerHTML = link_3.responseText;
                              $loader.querySelector(".is_paginate").insertAdjacentHTML('beforeend', elem.querySelector(".is_paginate").innerHTML);
                            }
                      }
                      link_3.send();
                  }
              };
            }
          };
      }
  };
  link.send();
};

function change_this_fullscreen(_this, $loader) {
    $loader.innerHTML = "";
    $parent_div.style.opacity = "0";
    $parent_div.style.height = "35px";
    url = _this.getAttribute("href");
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
    link.open('GET', url, true);
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  
    link.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            elem = link.responseText;
            $loader.innerHTML = elem;
            height = $loader.scrollHeight*1 + 30;
            $parent_div = $loader.parentElement
            if (height < 500 && !$loader.querySelector(".data_display")){
              $parent_div.style.height = height + "px";
              _height = (window.innerHeight - height - 50) / 2;
              $parent_div.style.top = _height + "px";
              prev_next_height = _height*1 + 50 + "px";
              $loader.style.overflowY = "unset";
            } else {
              $parent_div.style.height = "100%";
              $parent_div.style.top = "15px";
              $loader.style.overflowY = "auto";
            };
            $parent_div.style.opacity = "1";
            $parent_div.style.opacity = "1";
            if ($loader.querySelector(".data_display")) {
              $loader.style.overflowY = "unset";
            };
            $window_seconds = 1;
            get_document_opacity_0();
            window_time_end = false;
            offset = 0;
            //get_window_view_timer(120);
  
            $loader.onscroll = function() {
              //if ($window_time_end) {
              //  get_window_view_timer(120);
              //  $window_time_end = false;
              //};
              if ($loader.scrollHeight > offset) {
                  offset = $loader.scrollHeight;
                  $window_height = parseFloat(offset * 0.000264).toFixed(2);
                }
              if ($loader.querySelector(".next_page_list")) {
                box = $loader.querySelector('.next_page_list');
                if (box && box.classList.contains("next_page_list")) {
                    inViewport = elementInViewport(box);
                    if (inViewport) {
                        box.remove();
                        var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
                        link_3.open('GET', location.protocol + "//" + location.host + box.getAttribute("data-link"), true);
                        link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  
                        link_3.onreadystatechange = function() {
                            if (this.readyState == 4 && this.status == 200) {
                                var elem = document.createElement('span');
                                elem.innerHTML = link_3.responseText;
                                $loader.querySelector(".is_block_paginate").insertAdjacentHTML('beforeend', elem.querySelector(".is_block_paginate").innerHTML);
                              }
                        }
                        link_3.send();
                    }
                };
              };
            }
        }
    };
    link.send();
};

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

on('body', 'click', '.next_item', function(event) {
    event.preventDefault();
    this.style.display = "none";
    change_this_fullscreen(this, document.body.querySelector("#fullscreen_loader"));
  });
  on('body', 'click', '.prev_item', function(event) {
    event.preventDefault();
    this.style.display = "none";
    change_this_fullscreen(this, document.body.querySelector("#fullscreen_loader"));
});

on('body', 'click', '#logg', function() {
    _this = this;
    form = _this.parentElement.parentElement; 
    response = form.querySelector(".api_response");
    //linguage = document.getElementById("top").getAttribute("data-l")*1;
  
    if (!form.querySelector("#id_email").value){
      form.querySelector("#id_email").style.border = "1px #FF0000 solid";
      //if (linguage == 1) {
        response.innerHTML = "Enter your email!";
      //}
      //else {
      //  response.innerHTML = "Enter your username!";
      //}
      response.classList.add("error");
      return
    }
    else if (!form.querySelector("#id_password").value){
      form.querySelector("#id_password").style.border = "1px #FF0000 solid";
      //if (linguage == 1) {
        response.innerHTML = "Enter the password!";
      //}
      //else {
      //  response.innerHTML = "Enter your username!";
      //}
      response.classList.add("error")
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
      let data = JSON.stringify(link.response);
      //localStorage.setItem("key", data);
      console.log(data);
      console.log(data[1]);
      console.log(data["key"]);
      //window.location.href = "/" 
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
    response = form.querySelector(".api_response");
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
      rtr = document.getElementById('reload');
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


on('body', 'click', 'a', function(event) {
  event.preventDefault();
  //if (this.getAttribute("href") == window.location.pathname){
  //  toast_info("Вы уже на этой странице");
  //  return
  //};
  ajax_get_reload(this.getAttribute("href"), true, 2)
});