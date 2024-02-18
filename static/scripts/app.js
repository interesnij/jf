
//пока сокеты оставим неактивными
//connect() 

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
  ajax_link.open( 'GET', url, true );
  ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  if (localStorage.getItem('request_data') !== null) {
        ajax_link.setRequestHeader('Request-Data', localStorage.getItem('request_data'));
  }
  ajax_link.onreadystatechange = function () {
  if ( this.readyState == 4 && this.status == 200 ) {
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          span.append(elem_.innerHTML);
      } 
    }
    ajax_link.send();
});