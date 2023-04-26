<script>
    import {goto} from '$app/navigation';
    function rippy() {
    goto("/rippyroo")
}
</script>

<div class="centered">
<h1>
    Rippa Roo
</h1>
<button class="start" on:click={rippy}>Play</button>
</div>
<style>
    h1 {
        font-size: 10vw;
        text-shadow: 5px 5px #652828;
        color: rgb(255, 85, 0)    }
  
    .centered {
  position: fixed;
  top: 50%;
  left: 50%;
  /* bring your own prefixes */
  transform: translate(-50%, -50%);
  text-align: center;
}


.start {
        background: silver;
 
    height: 100px;
    width: 100px;
    font-size: 16px;
    text-align: center;
   
    font-weight: bold;
    cursor: pointer;
    box-shadow: inset -1px -1px #0a0a0a, inset 1px 1px #fff, inset -2px -2px grey, inset 2px 2px #dfdfdf;
    }

    .start:hover {
      border-width: -1px;
      background: rgb(185, 185, 185);
      
    }
    .start:active {
      box-shadow: inset -1px -1px #fff, inset 1px 1px #0a0a0a, inset -2px -2px #dfdfdf, inset 2px 2px grey;
       padding-bottom: 1px;
       padding-right: 1px;
   }


</style>