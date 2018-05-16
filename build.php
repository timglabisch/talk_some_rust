<?php

require __DIR__.'/vendor/autoload.php';

$loader = new Twig_Loader_Filesystem(__DIR__);
$twig = new Twig_Environment($loader);

$template = $twig->load('index.html.twig');

$content = $template->render();

file_put_contents('index.html', $content);