/// Template for a simple html viewport for some mocked data :3
pub const PROFILE_TEMPLATE: &str = r#"
<!doctype html>

<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">

<title>A Basic HTML5 Template</title>
<meta name="description" content="A basic HTML5 Template for new projects.">
<meta name="author" content="Woile">
</head>

<body>
<h1>Profile of {{ profile.full_name|title }}</h1>
<p>This is a template example to show some functionality</p>
<h2>Items</h3>
<ul>
{% for item in profile.items %}
<li>{{ item.name }} ({{ item.id }})</li>
{% endfor %}
<ul>
</body>
</html>
"#;

/// Template for a simple html viewport for some mocked data :3
pub const USERS_TEMPLATE: &str = r#"
<!doctype html>

<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">

<title>A Basic HTML5 Template</title>
<meta name="description" content="A basic HTML5 Template for new projects.">
<meta name="author" content="Woile">
</head>

<body>
<h1>Profile of {{ profile.full_name|title }}</h1>
<p>This is a template example to show some functionality</p>
<h2>Items</h3>
<ul>
{% for item in profile.items %}
<li>{{ item.name }} ({{ item.id }})</li>
{% endfor %}
<ul>
</body>
</html>
"#;
