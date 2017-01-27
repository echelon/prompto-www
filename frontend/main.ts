
import $ = require('jquery');
import Dropzone = require('dropzone');

console.log('loaded');

$(function() {
  /*let f = "asdf";
  f = 2;*/
  console.log('jquery loadeded!!!');


  $('form').on('submit', function(e) {
    let filename = $('#file').val();
    console.log('form submit');
    console.log(e);
    //let files = drop.getAcceptedFiles();
    //console.log('accepted files', files);
    //files = drop.getRejectedFiles();
    //console.log('rejected files', files);
    //files = drop.getQueuedFiles();
    //console.log('queued files', files);
    e.preventDefault();

		let file = getFile();

		let formData = new FormData();
		formData.append('file', file);

		xhrUpload(formData);

    //drop.processQueue();
  });

});

function xhrUpload(formData: FormData) {
	var xhr = new XMLHttpRequest();
	xhr.onreadystatechange = function(){
		if (this.readyState == 4 && this.status == 200){
			console.log('xhr state change');
			console.log(this.response, typeof this.response);

			//this.response is what you're looking for
			//handler(this.response);
			var img = document.getElementById('img');
			var url = window.URL || window.webkitURL;
			let url2 = url.createObjectURL(this.response);
			console.log('url', url);
			console.log('url2', url2);
			img.src = url2;

		}
	}

	xhr.open('POST', '/upload');
	xhr.responseType = 'blob';

	xhr.send(formData); 
}

function getFile() : File {
	let $file = $('#file')[0];
	return $file.files[0];
}

window.xhrUpload = xhrUpload;
window.getFile = getFile;

function configureDropzone() {
  let drop = new Dropzone("form", { url: "/upload"});

  //Dropzone.autoDiscover = false;
  //Dropzone.options.autoProcessQueue = true;

  //drop.options.previewsContainer = "#preview";
  drop.on('sending', function(file, xhr, formData) {
    console.log('sending');
  });

  drop.on('uploadprogress', function(file, progress, bytesSent) {
    console.log('uploadprogress');
  });

  drop.on('sending', function(file, xhr, formData) {
    console.log('sending');
  });

  drop.on('success', function(file, xhr) {
    console.log('success');
  });


  drop.on('complete', function(file) {
    console.log('done');
  });

  drop.on('addedfile', function(file) {
    console.log('addedfile:', file);
  });

  drop.on('error', function(file, errorMessage) {
    console.log('error:', file);
  });
}

    /*$.ajax({
      type: 'POST',
      url: '/upload',
      cache: false,
      contentType: false,
      processData: false,
      data: formData,
      success: function(data, textStatus, xhr) {
        console.log('success');
        console.log('data', data);
        console.log('status', textStatus);
        console.log('xhr', xhr);
      },
    });*/
