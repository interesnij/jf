
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

on('body', 'click', '.close_modal', function() {
  document.body.querySelector(".modal-control-container").remove()
});

on('body', 'click', '.close_full_modal', function() {
  document.body.querySelector(".full-modal__background").remove()
});

on('body', 'click', '.close_matter_client_container', function() {
  this.parentElement.remove()
});