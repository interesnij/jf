

//пока сокеты оставим неактивными
//connect() 

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
on('body', 'click', '.create_matter', function() {
  span = document.body.querySelector("#reload");
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/matter?ajax=2", true );
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
on('body', 'click', '.create_document', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/document?ajax=2", true );
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
on('body', 'click', '.create_expense_entry', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/expense_entry?ajax=2", true );
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
on('body', 'click', '.create_time_entry', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/time_entry?ajax=2", true );
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
on('body', 'click', '.create_flat_fee', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/flat_fee?ajax=2", true );
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
on('body', 'click', '.create_invoice', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/invoice?ajax=2", true );
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
on('body', 'click', '.create_template', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/template?ajax=2", true );
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
on('body', 'click', '.create_post', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/post?ajax=2", true );
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
on('body', 'click', '.create_note', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/note?ajax=2", true );
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
on('body', 'click', '.create_invoice', function() {
  span = document.body.querySelector("#reload"); 
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/create/invoice?ajax=2", true );
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
      } 
    }
    ajax_link.send();
});
on('body', 'click', '.account_settings', function() {
  span = document.body.querySelector("#reload"); 
  if (localStorage.getItem('request_data') !== null) {
    request_data = localStorage.getItem('request_data');
    ajax_link.setRequestHeader('Request-Data', request_data);
    user_type = request_data.user_type;
  }
  ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  ajax_link.open( 'GET', "/" + user_type + "/settings/account?ajax=2", true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
   
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          span.append(elem_);
      } 
    }
    ajax_link.send();
});


//////////////////////////////////////
/////////// auth function ////////

function get_register_2_step_low(url) {
    /*
      reg_step_1 : { 'email': _email.value, 'password': _password1.value}
    */

    form = document.body.querySelector(".js_form");
    _email = form.querySelector("#id_email");
    _password1 = form.querySelector("#id_password");
    _password2 = form.querySelector("#id_password2");

    _email.style.setProperty('border', 'inherit', 'important');
    _password1.style.setProperty('border', 'inherit', 'important');
    _password2.style.setProperty('border', 'inherit', 'important');

    if (!_email.value){
      _email.style.border = "1px #FF0000 solid";
      toast_error("Email is required");
      return
    } else if (!_password1.value){
      _password1.style.border = "1px #FF0000 solid";
      toast_error("Password is required");
      return
    }
    else if (!_password2.value){
      _password2.style.border = "1px #FF0000 solid";
      toast_error("Password is required");
      return
    }

    else if (_password1.value != _password2.value){
      _password1.style.border = "1px #FF0000 solid";
      _password2.style.border = "1px #FF0000 solid";
      toast_error("Passwords must match");
      return
    }

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

on('body', 'click', '.add_jurisdiction_form', function() {
  block = this.parentElement.previousElementSibling;
  block.innerHTML = '<div class="d-flex col-12 w-100 mt-0 juri_block"><div class="flex-1"><div class="row"><div class="select-control col-md-3 mt-2 load_content1 load_countries" data-link="/load/countries"></div><div class="select-control col-md-3 mt-2 states_container"></div><div class="input-control col-md-3 mt-2"><div class="d-flex justify-content-between align-items-center"><label for="id_number" class="input-control__label">Registration Number</label></div><input name="number" class="id_number" placeholder="Enter Registration Number" type="text"><div class="input-control__footer"></div></div><div class="input-control col-md-3 mt-2"><div class="d-flex justify-content-between align-items-center"><label class="input-control__label">Year Admitted</label></div><input name="year" class="id_year" placeholder="Enter Registration Number" type="number"><div class="input-control__footer"></div></div></div></div><img class="sc-jXktde kxdlOK mb-auto close_juri_block" src="/static/images/close.svg"></div><span></span>';
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
  //load_data(1, block);
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
  _last_name.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
  _phone.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
  _attachments.style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
  try {
      for (let i = 0; i < _jurisdictions_countries.length; i++) {
        _jurisdictions_countries[i].style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
      }
      for (let i = 0; i < _jurisdictions_numbers.length; i++) {
        _jurisdictions_numbers[i].style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
      }
      for (let i = 0; i < _jurisdictions_years.length; i++) {
        _jurisdictions_years[i].style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
      }
      for (let i = 0; i < _jurisdictions_states.length; i++) {
        _jurisdictions_states[i].style.setProperty('border', '1px solid rgba(0, 0, 0, 0.25)', 'important');
      }
  } catch { null }
  /////

  if (!_first_name.value){
    _first_name.style.border = "1px #FF0000 solid";
    toast_error("First Name is required");
    return
  } else if (!_last_name.value){
    _last_name.style.border = "1px #FF0000 solid";
    toast_error("Last Name is required");
    return
  }
  else if (!_phone.value){
    _phone.style.border = "1px #FF0000 solid";
    toast_error("Phone is required");
    return
  }
  else if (!_attachments.value){
    _attachments.parentElement.style.border = "1px #FF0000 solid";
    toast_error("Registration Attachments is required");
    return
  }
  else if (!_jurisdictions_countries[0].value){
    _jurisdictions_countries[0].value.style.border = "1px #FF0000 solid";
    toast_error("Jurisdiction Country is required");
    return
  }
  else if (!_jurisdictions_numbers[0].value){
    _jurisdictions_numbers[0].value.style.border = "1px #FF0000 solid";
    toast_error("Jurisdiction Registration Number is required");
    return
  }
  else if (!_jurisdictions_years[0].value){
    _jurisdictions_years[0].value.style.border = "1px #FF0000 solid";
    toast_error("Jurisdiction Year Admitted is required");
    return
  }

  jurisdictions = [];
  for (let i = 0; i < _jurisdictions_numbers.length; i++) {
    try {
      let country, state, city, number, year;
      country = _jurisdictions_countries[i].value;
      state = _jurisdictions_states[i].value;
      city = _jurisdictions_cities[i].value;
      number = _jurisdictions_numbers[i].value;
      year = _jurisdictions_years[i].value;
      jurisdictions.append({
        country: country, 
        state: state,
        city: city,
        number: number, 
        year: year
      });
    } catch { 
      if (i == 0) {
        return
      }
      else {
        break
      }
    }
  }

  files = send_files(_attachments);

  fObject = {
    "email": email,
    "password1": password1,
    "password2": password2,
    "first_name": _first_name.value,
    "last_name": _last_name.value,
    "phone": _phone.value,
    "is_disciplined": form.elements["disciplined"],
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
          ajax_link.setRequestHeader('Request-Data', localStorage.getItem('request_data'));
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
          ajax_link.setRequestHeader('Request-Data', localStorage.getItem('request_data'));
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
        } 
      }
      ajax_link.send();
});

on('body', 'change', '.state', function() {
  _this = this;
  val = _this.value;
  if (val == '' || _this.parentElement.classList.contains("no_cities_load") ) {
    return
  } 
  
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
      } 
    }
    ajax_link.send();
});

on('body', 'change', '.firm_plus_input', function(event) {
  block = this.parentElement.parentElement.parentElement.parentElement.nextElementSibling;
  if (event.currentTarget.checked) {
    block.classList.remove("hidden");
  } else {
    block.classList.add("hidden");
  }
});