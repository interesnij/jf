

//пока сокеты оставим неактивными
//connect()

on('body', 'click', '.multi_select_h', function(e) {
  block = this.parentElement.nextElementSibling;
  //if (e.target.classList.contains("close_multi_item")) {}
  console.log(e.target.classList);
  if (block.contains("open")) {
    block.remove("open");
    block.style.opacity = "0";
    block.style.visibility = "hidden";
  } else {
    block.add("open");
    block.style.opacity = "1";
    block.style.visibility = "visible";
  }
});

on('body', 'click', '.multi-select-control__menu-item', function() {
  block = this.parentElement.previousElementSibling;
  pk = this.getAttribute("data-pk");
  if (block.querySelector( '[data-pk=' + '"' + pk + '"' + ']' )) {
      this.classList.remove("active");
      name_c = block.querySelector( '[data-pk=' + '"' + pk + '"' + ']' );
      name_c.parentElement.remove();
      block.querySelector(".hidden_input" + pk).remove();
  } else {
      this.classList.remove("active");
      input = document.createElement("input");
      input.setAttribute("value", pk);
      input.setAttribute("type", "hidden");
      input.setAttribute("name", block.getAttribute("data-name"));
      input.classList.add(".hidden_input" + pk);
      block.append(input);
      item = '<div class="multi-select-control__item"><span class="my-auto" data-id="' 
      + pk +
      '">'
      + this.querySelector("span").innerHTML +
      '</span><img src="/static/images/close.svg" alt="close" class="multi-select-control__item-close close_multi_item"></div>';
  }
});

on('body', 'click', '.menu-button', function() {
  block = this.nextElementSibling;
  if (this.classList.contains("open")) {
    this.classList.remove("open");
    block.style.opacity = "0";
    block.style.visibility = "hidden";
  }
  if (this.classList.contains("open")) {
    this.classList.add("open")
    block.style.opacity = "1";
    block.style.visibility = "visible";
  }
});
on('body', 'click', '.profile-drop-down', function() {
  if (this.classList.contains("hide")) {
    this.classList.remove("hide");
    this.classList.add("show");
  } else {
    this.classList.add("hide");
    this.classList.remove("show");
  }
});

on('body', 'click', '#logg', function() {
  _this = this;
  form = _this.parentElement.parentElement; 
  email = form.querySelector("#id_email");
  password = form.querySelector("#id_password");

  if (!email.value){
    email.style.border = "1px #FF0000 solid";
    email.nextElementSibling.innerHTML = "Enter your email!";
    return
  }
  else if (!password.value){
    password.style.border = "1px #FF0000 solid";
    password.nextElementSibling.innerHTML = "Enter the password!";
    return
  }
  else {
    _this.disabled = true;
  }

  form_data = new FormData(form);
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "https://backend.juslaw.com/api/v1/auth/login/", true );

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    localStorage.setItem("request_data", link.response);
    window.location.href = "/";
  }

  else {
    _this.disabled = false;
  }};
  link.send(form_data);
});

on('body', 'click', '.ajax', function(event) {
  event.preventDefault();
  ajax_get_reload(this.getAttribute("href"), true, 2);
});

on('body', 'click', '.create_contact', function() {
  span = document.body.querySelector("#reload");
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/contact?ajax=2", true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  if (localStorage.getItem('request_data') !== null) {
      ajax_link.setRequestHeader('Request-Data', JSON.parse(localStorage.getItem('request_data')));
  } 
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          span.append(elem_);
      } 
    }
    ajax_link.send();
});
on('body', 'click', '.create_matter', function() {
  span = document.body.querySelector("#reload");
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/matter?ajax=2", true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  if (localStorage.getItem('request_data') !== null) {
      //ajax_link.setRequestHeader('Request-Data', JSON.parse(localStorage.getItem('request_data')));
      ajax_link.setRequestHeader('Request-Data', localStorage.getItem('request_data'));
  } 
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          span.append(elem_);
          load_data(0, elem_);
      } 
    }
    ajax_link.send();
});
on('body', 'click', '.create_document', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/document?ajax=2", true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  if (localStorage.getItem('request_data') !== null) {
    ajax_link.setRequestHeader('Request-Data', JSON.parse(localStorage.getItem('request_data')));
  } 
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          span.append(elem_);
      } 
    }
    ajax_link.send();
});
on('body', 'click', '.create_expense_entry', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/expense_entry?ajax=2", true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  if (localStorage.getItem('request_data') !== null) {
    ajax_link.setRequestHeader('Request-Data', JSON.parse(localStorage.getItem('request_data')));
  } 
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          span.append(elem_);
      } 
    }
    ajax_link.send();
});
on('body', 'click', '.create_time_entry', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/time_entry?ajax=2", true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  if (localStorage.getItem('request_data') !== null) {
    ajax_link.setRequestHeader('Request-Data', JSON.parse(localStorage.getItem('request_data')));
  } 
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          span.append(elem_);
      } 
    }
    ajax_link.send();
});
on('body', 'click', '.create_flat_fee', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/flat_fee?ajax=2", true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  if (localStorage.getItem('request_data') !== null) {
    ajax_link.setRequestHeader('Request-Data', JSON.parse(localStorage.getItem('request_data')));
  } 
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          span.append(elem_);
      } 
    }
    ajax_link.send();
});
on('body', 'click', '.create_invoice', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/invoice?ajax=2", true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  if (localStorage.getItem('request_data') !== null) {
    ajax_link.setRequestHeader('Request-Data', JSON.parse(localStorage.getItem('request_data')));
  } 
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          span.append(elem_);
      } 
    }
    ajax_link.send();
});
on('body', 'click', '.create_template', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/template?ajax=2", true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  if (localStorage.getItem('request_data') !== null) {
    ajax_link.setRequestHeader('Request-Data', JSON.parse(localStorage.getItem('request_data')));
  } 
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          span.append(elem_);
      } 
    }
    ajax_link.send();
});
on('body', 'click', '.create_post', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/post?ajax=2", true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  if (localStorage.getItem('request_data') !== null) {
    ajax_link.setRequestHeader('Request-Data', JSON.parse(localStorage.getItem('request_data')));
  } 
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          span.append(elem_);
      } 
    }
    ajax_link.send();
});
on('body', 'click', '.create_note', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/note?ajax=2", true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  if (localStorage.getItem('request_data') !== null) {
    ajax_link.setRequestHeader('Request-Data', JSON.parse(localStorage.getItem('request_data')));
  } 
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          span.append(elem_);
      } 
    }
    ajax_link.send();
});
on('body', 'click', '.create_invoice', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/invoice?ajax=2", true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  if (localStorage.getItem('request_data') !== null) {
    ajax_link.setRequestHeader('Request-Data', JSON.parse(localStorage.getItem('request_data')));
  } 
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          span.append(elem_);
      } 
    }
    ajax_link.send();
});
on('body', 'click', '.create_chat', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/chat?ajax=2", true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  if (localStorage.getItem('request_data') !== null) {
        ajax_link.setRequestHeader('Request-Data', localStorage.getItem('request_data'));
  } 
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          span.append(elem_);
      } 
    }
    ajax_link.send();
});

on('body', 'click', '.close_modal', function() {
  document.body.querySelector(".modal-control-container").remove()
});

on('body', 'click', '.close_full_modal', function() {
  document.body.querySelector(".full-modal__background").remove()
});

on('body', 'click', '.close_matter_client_container', function() {
  this.parentElement.remove()
});

on('body', 'click', '.logout_hundler', function(e) {
  e.preventDefault();
  console.log("click logout_hundler");
  localStorage.clear();
  window.location.href = "/";
});

on('body', 'click', '.profile_settings', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/attorney/settings/profile?ajax=2", true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  if (localStorage.getItem('request_data') !== null) {
        ajax_link.setRequestHeader('Request-Data', localStorage.getItem('request_data'));
  } 
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          span.append(elem_);
          load_data(0, elem_);
      } 
    }
    ajax_link.send();
});
on('body', 'click', '.account_settings', function() {
  span = document.body.querySelector("#reload"); 
  request_data = localStorage.getItem('request_data');
  console.log(request_data);
  user_type = request_data.user_type;
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/" + user_type + "/settings/account?ajax=2", true );
  ajax_link.setRequestHeader('Request-Data', JSON.parse(request_data));
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
   
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          span.append(elem_);
          load_data(0, elem_);
      } 
    }
    ajax_link.send();
});


//////////////////////////////////////
/////////// auth function ////////

function send_files(file_c) {
  file_data = new FormData();
  file_data.append("token", "111");
  file_data.append("folder", "111");
  file_data.append("object_id", "111");
  files = file_c.files;
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
      return
    }
    else {
      return res_files[0];
    }
  } else {
    console.log("return");
    return
  }};
  _link.send(file_data);
}

function get_register_2_step_low(url) {
    /*
      reg_step_1 : { 'email': _email.value, 'password': _password1.value}
    */

    form = document.body.querySelector(".js_form");
    _email = form.querySelector("#id_email");
    _password1 = form.querySelector("#id_password");
    _password2 = form.querySelector("#id_password2");

    _email.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
    _email.nextElementSibling.innerHTML = "";

    _password1.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
    _password1.nextElementSibling.innerHTML = "";

    _password2.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
    _password2.nextElementSibling.innerHTML = "";

    let is_error = false;
    if (!_email.value){
      _email.style.border = "1px #FF0000 solid";
      _email.nextElementSibling.innerHTML = "Email is required";
      is_error = true;
    };
    if (!_password1.value){
      _password1.style.border = "1px #FF0000 solid";
      _password1.nextElementSibling.innerHTML = "Password is required";
      is_error = true;
    };
    if (!_password2.value){
      _password2.style.border = "1px #FF0000 solid";
      _password2.nextElementSibling.innerHTML = "Repeat the password";
      is_error = true;
    };

    if (_password1.value != _password2.value){
      _password1.style.border = "1px #FF0000 solid";
      _password2.style.border = "1px #FF0000 solid";
      _password2.nextElementSibling.innerHTML = "Passwords must match";
      return 
    };
    if (is_error) {
      return
    };
    tObject = { 'email': _email.value, 'password': _password1.value};
    localStorage.setItem('reg_step_1', JSON.stringify(tObject));
    ajax_get_reload(url, true, 2);
}

function back_register_1_step_low(url) {
    /*
      reg_step_1 : { 'email': _email.value, 'password': _password1.value}
    */
      ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'GET', url + "?ajax=2", true );
      ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
          meta_block = document.body.querySelector('.doc_title');
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          meta_block.innerHTML = elem_.innerHTML;

          _email = meta_block.querySelector("#id_email");
          _password1 = meta_block.querySelector("#id_password");
          _password2 = meta_block.querySelector("#id_password2");
        
          _tObject = localStorage.getItem('reg_step_1');
          tObject = JSON.parse(_tObject);
          console.log("_email", _email);
          console.log("tObject email", tObject.email);
          console.log("password", tObject.password);

          _email.value = tObject.email;
          _email.value = "fbfbfbfbfbfbfbfb";
          _password1.value = tObject.password;
          _password2.value = tObject.password;
          
          try { 
            $link = document.location.pathname;
            meta_block = document.body.querySelector(".doc_title");
            $title = meta_block.getAttribute("data-title");
            
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
          window.scrollTo(0,0);
          window.history.pushState ({"url":url}, $title, url);
        }
      }
      ajax_link.send();
}

//////////////////////

on('body', 'click', '.register_2_client_btn', function() {
  get_register_2_step_low("/auth/register_client_2");
});
on('body', 'click', '.back_register_1_client_btn', function() {
  back_register_1_step_low("/auth/register/client");
});
on('body', 'click', '.register_2_attorney_btn', function() {
  get_register_2_step_low("/auth/register_attorney_2");
});
on('body', 'click', '.back_register_1_attorney_btn', function() {
  back_register_1_step_low("/auth/register/attorney");
});
on('body', 'click', '.register_2_paralegal_btn', function() {
  get_register_2_step_low("/auth/register_paralegal_2");
});
on('body', 'click', '.back_register_1_paralegal_btn', function() {
  back_register_1_step_low("/auth/register/paralegal");
});
on('body', 'click', '.register_2_enterprise_btn', function() {
  get_register_2_step_low("/auth/register_enterprise_2");
});
on('body', 'click', '.back_register_1_enterprise_btn', function() {
  back_register_1_step_low("/auth/register/enterprise");
});
///////////////////////////

on('body', 'click', '.onboard_2_attorney_btn', function() {
    ajax_get_reload("/auth/onboard_attorney_2", true, 2, "onboarding-page")
});
on('body', 'click', '.onboard_3_attorney_btn', function() {
  ajax_get_reload("/auth/onboard_attorney_3", true, 2, "onboarding-page")
}); 
on('body', 'click', '.onboard_4_attorney_btn', function() {
  ajax_get_reload("/auth/onboard_attorney_4", true, 2, "onboarding-page")
});

on('body', 'click', '.back_onboard_1_attorney_btn', function() {
  ajax_get_reload("/auth/onboard_attorney_1", true, 2, "onboarding-page")
});
on('body', 'click', '.back_onboard_2_attorney_btn', function() {
  ajax_get_reload("/auth/onboard_attorney_2", true, 2, "onboarding-page")
});
on('body', 'click', '.back_onboard_3_attorney_btn', function() {
  ajax_get_reload("/auth/onboard_attorney_3", true, 2, "onboarding-page")
});


on('body', 'click', '.register_final_attorney_btn', function() {
  if (localStorage.getItem("reg_step_1") === null) {
    back_register_1_step_low("/auth/register/attorney");
  }

  _tObject = localStorage.getItem('reg_step_1');
  tObject = JSON.parse(_tObject);

  form = document.body.querySelector(".js_form");
  _first_name = form.querySelector("#id_first_name");
  _last_name = form.querySelector("#id_last_name");
  _phone = form.querySelector("#id_phone");
  _attachments = form.querySelector("#id_attachments");
  email = tObject.email;
  password1 = tObject.password;
  password2 = tObject.password;  

  _jurisdictions_countries = form.querySelectorAll(".country");
  _jurisdictions_states = form.querySelectorAll(".state");
  _jurisdictions_numbers = form.querySelectorAll(".id_number");
  _jurisdictions_years = form.querySelectorAll(".id_year");

  // return normal fields styles 
  _first_name.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
  _first_name.nextElementSibling.innerHTML = "";
  
  _last_name.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
  _last_name.nextElementSibling.innerHTML = "";

  _phone.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
  _phone.nextElementSibling.innerHTML = "";

  _attachments.parentElement.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
  _attachments.parentElement.querySelector(".attachments_error").innerHTML = "";

  try {
      for (let i = 0; i < _jurisdictions_countries.length; i++) {
        _jurisdictions_countries[i].style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
        _jurisdictions_countries[0].nextElementSibling.nextElementSibling.innerHTML = "";
      }
      for (let i = 0; i < _jurisdictions_numbers.length; i++) {
        _jurisdictions_numbers[i].style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
        _jurisdictions_numbers[0].nextElementSibling.innerHTML = "";
      }
      for (let i = 0; i < _jurisdictions_years.length; i++) {
        _jurisdictions_years[i].style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
        _jurisdictions_years[0].nextElementSibling.innerHTML = "";
      }
      for (let i = 0; i < _jurisdictions_states.length; i++) {
        _jurisdictions_states[i].style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
        _jurisdictions_states[0].nextElementSibling.nextElementSibling.innerHTML = "";
      }
  } catch { null }
  /////

  let is_error = false;
  if (!_first_name.value){
    _first_name.style.border = "1px #FF0000 solid";
    _first_name.nextElementSibling.innerHTML = "First Name is required";
    is_error = true;
  };
  if (!_last_name.value){
    _last_name.style.border = "1px #FF0000 solid";
    _last_name.nextElementSibling.innerHTML = "Last Name is required";
    is_error = true;
  }; 
  if (!_phone.value){
    _phone.style.border = "1px #FF0000 solid";
    _phone.nextElementSibling.innerHTML = "Phone is required";
    is_error = true;
  };
  if (!_attachments.value){
    parent = _attachments.parentElement;
    parent.style.border = "1px #FF0000 solid";
    parent.querySelector(".attachments_error").innerHTML = "Registration Attachments is required";
    is_error = true;
  };
  if (!_jurisdictions_countries[0].value){
    _jurisdictions_countries[0].style.border = "1px #FF0000 solid";
    _jurisdictions_countries[0].nextElementSibling.nextElementSibling.innerHTML = "Country is required";
    is_error = true;
  };
  console.log(_jurisdictions_states[0]);
  if (_jurisdictions_states[0] != undefined && !_jurisdictions_states[0].value){
    _jurisdictions_states[0].style.border = "1px #FF0000 solid";
    _jurisdictions_states[0].nextElementSibling.nextElementSibling.innerHTML = "State is required";
    is_error = true;
  };
  if (!_jurisdictions_numbers[0].value){
    _jurisdictions_numbers[0].style.border = "1px #FF0000 solid";
    _jurisdictions_numbers[0].nextElementSibling.innerHTML = "Registration Number is required";
    is_error = true;
  };
  if (!_jurisdictions_years[0].value){
    _jurisdictions_years[0].style.border = "1px #FF0000 solid";
    _jurisdictions_years[0].nextElementSibling.innerHTML = "Year Admitted is required";
    is_error = true;
  };
  console.log("is_error", is_error);
  if (is_error) {
    return;
  }

  jurisdictions = [];
  for (let i = 0; i < _jurisdictions_numbers.length; i++) {
    //try {
      let country, state, number, year;
      country = _jurisdictions_countries[i].getAttribute("data-pk");
      state = _jurisdictions_states[i].getAttribute("data-pk");
      number = _jurisdictions_numbers[i].value;
      year = _jurisdictions_years[i].value;
      jurisdictions.push({
        country: country, 
        state: state,
        number: number, 
        year: year
      });
    //} catch { null }
  }; 
  
  /////
  file_data = new FormData();
    file_data.append("token", "111");
    file_data.append("folder", "111");
    file_data.append("object_id", "111");
    files = _attachments.files;
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
        }
        else {
            files = res_files;
        }
    } else {
        console.log("return");
    }};
    _link.send(file_data);

  ////
 
  //files = send_files(_attachments);
  is_disciplined = false;
  if(form.querySelector('#disciplined_true').checked) {
    is_disciplined = true;
  }
  console.log("_jurisdictions_numbers ", _jurisdictions_numbers);
  console.log("_jurisdictions_numbers length ", _jurisdictions_numbers.length);
  console.log("jurisdictions ", jurisdictions);
  console.log("files ", files);
  console.log("disciplined ", is_disciplined);

  fObject = {
    "email": email,
    "password1": password1,
    "password2": password2,
    "first_name": _first_name.value,
    "last_name": _last_name.value,
    "phone": _phone.value,
    "is_disciplined": is_disciplined,
    "practice_jurisdictions": jurisdictions,
    "registration_attachments": files
  }
  
  localStorage.setItem('reg_step_2', JSON.stringify(fObject));
  fetch('https://backend.juslaw.com/api/v1/users/attorneys/', {
    method: 'POST',
    headers: {
      'Accept': 'application/json, text/plain, */*',
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(fObject)
  }).then(res => res.json())
    .then(res => show_law_reg_success_modal());
});

  //////////////////////////////////////
  //////////////////////////////////////

  on('body', 'click', '.select_double_content', function() {
    _this = this;
    if (_this.classList.contains("active")) {
      return
    };

    menu_items = _this.parentElement.querySelectorAll(".menu-item");
    
    for (let i = 0; i < menu_items.length; i++) {
      menu_items[i].classList.remove("active");
    } 
    _this.classList.add("active");

    block = document.body.querySelector(".load_content");
    _this.parentElement.parentElement.querySelector("span").innerHTML = _this.innerHTML;
    search_input = document.body.querySelector(".search_input");

    link = search_input.getAttribute("data-link") + "?search=" + search_input.value;
    active_menu_items = document.body.querySelectorAll(".menu-item.active");
    for (let i = 0; i < active_menu_items.length; i++) {
      link += active_menu_items[i].getAttribute("data-value");
    }

    ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    ajax_link.open( 'GET', link + "&ajax=2", true );
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    if (localStorage.getItem('request_data') !== null) {
      ajax_link.setRequestHeader('Request-Data', JSON.parse(localStorage.getItem('request_data')));
    } 
    ajax_link.onreadystatechange = function () {
    if ( this.readyState == 4 && this.status == 200 ) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            block.innerHTML = elem_.innerHTML;
        } 
      }
      ajax_link.send();
  });

on('body', 'click', '.select_content', function() {
    _this = this;
    if (_this.classList.contains("active")) {
      return
    };

    menu_items = _this.parentElement.querySelectorAll(".menu-item");
    
    for (let i = 0; i < menu_items.length; i++) {
      menu_items[i].classList.remove("active");
    } 
    _this.classList.add("active");

    block = document.body.querySelector(".load_content");
    _this.parentElement.parentElement.querySelector("span").innerHTML = _this.innerHTML;
    search_input = document.body.querySelector(".search_input");

    link = _this.getAttribute("data-link") + "&search=" + search_input.value;

    ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    ajax_link.open( 'GET', link + "&ajax=2", true );
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    if (localStorage.getItem('request_data') !== null) {
      ajax_link.setRequestHeader('Request-Data', JSON.parse(localStorage.getItem('request_data')));
    } 
    ajax_link.onreadystatechange = function () {
    if ( this.readyState == 4 && this.status == 200 ) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            block.innerHTML = elem_.innerHTML;
        } 
      }
      ajax_link.send();
});

on('body', 'change', '.country', function() {
    _this = this;
    val = _this.value;
    if (val == '') {
      return
    }
    option = _this.nextElementSibling.querySelector('[value=' + '"' + val + '"' + ']')
    pk = option.getAttribute("data-pk");
    
    block = _this.parentElement.parentElement.parentElement.querySelector(".states_container");
    ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    ajax_link.open( 'GET', "/load/states/" + pk + "?ajax=2", true );
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function () {
    if ( this.readyState == 4 && this.status == 200 ) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            block.innerHTML = elem_.innerHTML;
            _this.setAttribute("data-pk", pk);
        } 
      }
      ajax_link.send();
});

on('body', 'change', '.state', function() {
  _this = this;
  val = _this.value;
  if (val == '') {
    return
  };
  option = _this.nextElementSibling.querySelector('[value=' + '"' + val + '"' + ']')
  pk = option.getAttribute("data-pk");

  if (_this.parentElement.classList.contains("no_cities_load") ) {
    _this.setAttribute("data-pk", pk);
    return
  };
  
  option = _this.nextElementSibling.querySelector('[value=' + '"' + val + '"' + ']')
  pk = option.getAttribute("data-pk");

  block = _this.parentElement.parentElement.parentElement.querySelector(".cities_container");
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/load/cities/" + pk + "?ajax=2", true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          block.innerHTML = elem_.innerHTML;
          _this.setAttribute("data-pk", pk);
      } 
    }
    ajax_link.send();
});

on('body', 'change', '.city', function() {
    _this = this;
    val = _this.value;
    if (val == '') {
      return
    };
    option = _this.nextElementSibling.querySelector('[value=' + '"' + val + '"' + ']')
    pk = option.getAttribute("data-pk");
    _this.setAttribute("data-pk", pk);
    return
}); 

on('body', 'click', '.upload_avatar_c', function(event) {
  block = this.previousElementSibling;
  get_image_priview(block.querySelector(".user-avatar"), block.querySelector("#id_avatar"));
});

on('body', 'click', '.remove_avatar_c', function() {
  block = this.previousElementSibling.previousElementSibling;
  block.querySelector("#id_avatar").value = '';
  block.querySelector(".user-avatar").innerHTML = '<svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 448 512" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><path d="M224 256c70.7 0 128-57.3 128-128S294.7 0 224 0 96 57.3 96 128s57.3 128 128 128zm89.6 32h-16.7c-22.2 10.2-46.9 16-72.9 16s-50.6-5.8-72.9-16h-16.7C60.2 288 0 348.2 0 422.4V464c0 26.5 21.5 48 48 48h352c26.5 0 48-21.5 48-48v-41.6c0-74.2-60.2-134.4-134.4-134.4z"></path></svg>';
});


on('body', 'click', '.firm_plus_input', function(event) {
  block = this.parentElement.parentElement.parentElement.parentElement.nextElementSibling;
  if (event.currentTarget.checked) {
    block.classList.remove("hidden");
  } else {
    block.classList.add("hidden");
  }
});


on('body', 'click', '.add_jurisdiction_form', function() {
  block = this.parentElement.previousElementSibling;
  block.innerHTML = '<div class="d-flex col-12 w-100 mt-0 juri_block"><div class="flex-1"><div class="row"><div class="input-control col-md-3 mt-2 load_content1 load_countries" data-link="/load/countries"></div><div class="input-control col-md-3 mt-2 states_container"></div><div class="input-control col-md-3 mt-2"><div class="d-flex justify-content-between align-items-center"><label for="id_number" class="input-control__label">Registration Number</label></div><input name="number" class="id_number" placeholder="Enter Registration Number" type="text"><div class="input-control__footer"></div></div><div class="input-control col-md-3 mt-2"><div class="d-flex justify-content-between align-items-center"><label class="input-control__label">Year Admitted</label></div><input name="year" class="id_year" placeholder="Enter Registration Number" type="number"><div class="input-control__footer"></div></div></div></div><img class="sc-jXktde kxdlOK mb-auto close_juri_block" src="/static/images/close.svg"></div><span></span>';
  load_data(1, block);
}); 
on('body', 'click', '.add_firm_location_form', function() {
  block = this.parentElement.previousElementSibling;
  block.innerHTML = '<div class="d-flex flex-column w-100 pt-2"><div class="d-flex pr-3"></div><div class="col-12"><div class="row"><div class="input-control col-md-4 mt-2 load_content1 load_countries" data-link="/load/countries"></div><div class="input-control col-md-8"><div class="d-flex justify-content-between align-items-center"><label class="input-control__label">Address</label></div><input class="id_address" name="address" placeholder="Enter an address here" type="text"><div class="input-control__footer"></div></div><div class="input-control col-md-4 mt-2 states_container"></div><div class="input-control col-md-4 mt-2 cities_container"></div></div></div></div><img class="sc-jXktde kxdlOK mb-auto close_juri_block" src="/static/images/close.svg"></div><span></span>';
  load_data(1, block);
});
on('body', 'click', '.add_education_form', function() {
  block = this.parentElement.previousElementSibling;
  block.innerHTML = '<div class="d-flex w-100 col-12 mt-0"><div class="flex-1"><div class="row"><div class="input-control col-md-6 mt-2"><div class="d-flex justify-content-between align-items-center"><label class="input-control__label">Law school / Graduate Institute</label></div><input name="school"  class="school" placeholder="Enter a school / graduate institute name" type="text" class="school active"><div class="input-control__footer"></div></div><div class="input-control col-md-6 mt-2"><div class="d-flex justify-content-between align-items-center"><label class="input-control__label">Years</label></div><input name="year" placeholder="Enter number of years" type="number" class="active year" value="1"><div class="input-control__footer"></div></div></div></div></div><img class="sc-jXktde kxdlOK mb-auto close_juri_block" src="/static/images/close.svg"></div><span></span>';
});

on('body', 'click', '.select-control__menu-item', function() {
  _this = this;
  c = this.parentElement.previousElementSibling;
  c.querySelector("span").innerHTML = _this.innerHTML;
  c.querySelector(".hidden_input").value = _this.getAttribute("data-pk");
  menu_items = _this.parentElement.querySelectorAll(".select-control__menu-item");
  for (let i = 0; i < menu_items.length; i++) {
    menu_items[i].classList.remove("active");
  } 
  _this.classList.add("active");
});

function show_law_reg_success_modal() {
  span = document.body.querySelector("#reload");
  span2 = document.createElement("span");
  span2.innerHTML = '<div class="modal-control-container open"><div tabindex="-1" class="modal-control"><div class="modal-control__header"><div class="my-auto title text-ellipsis w-100 text-center">Application Received</div></div><div class="modal-control__content ignore-onclickoutside"><div class="pb-4" style="width: 600px;"><div class="text-black" style="font-size: 18px;">JusLaw has received your application.</div><br><div class="text-dark">The verification process of your application will take up to 5 days. Please check your inbox for an email from JusLaw to verify your email in the meantime. If your application is approved, our team will guide you to set up your account.</div></div><div class="d-flex mt-4"><a class="btn btn--green ripple-effect normal ml-auto return_login_hundler"><span>Return Home</span></a></div></div></div></div>';
  span.append(span2); 
}
on('body', 'click', '.return_login_hundler', function() {
    document.body.querySelector(".modal-control-container").remove();
    ajax_get_reload("/", true, 2);
});

on('body', 'click', '.close_juri_block', function() {
  this.parentElement.remove();
});

on('body', 'click', '.select_files', function() {
  this.previousElementSibling.previousElementSibling.click();
});

on('body', 'change', '#id_attachments', function() {
  console.log('Selected file: ' + this.value);
  files = this.files;
  len = files.length;
  if (len > 10) {
    alert("The maximum number of photos is 10")
  }
  for (let i = 0; i < len; i++) {
    console.log("file", files[i].name);
    //file_data.append("file", files[i]);
  }
}); 