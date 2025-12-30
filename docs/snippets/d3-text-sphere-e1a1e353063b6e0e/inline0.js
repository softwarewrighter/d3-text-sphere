
    export function check_d3_available() {
        return typeof d3 !== 'undefined';
    }

    export function create_svg(container_id, width, height) {
        const svg = d3.select('#' + container_id)
            .append('svg')
            .attr('width', width)
            .attr('height', height)
            .attr('id', 'main-svg');

        // Add gradient for sphere
        const defs = svg.append('defs');
        const gradient = defs.append('radialGradient')
            .attr('id', 'sphere-gradient')
            .attr('cx', '35%')
            .attr('cy', '35%')
            .attr('r', '60%');

        gradient.append('stop')
            .attr('offset', '0%')
            .attr('stop-color', '#6699ff');
        gradient.append('stop')
            .attr('offset', '70%')
            .attr('stop-color', '#3366cc');
        gradient.append('stop')
            .attr('offset', '100%')
            .attr('stop-color', '#1a3366');

        return svg.node();
    }

    export function update_svg_size(width, height) {
        d3.select('#main-svg')
            .attr('width', width)
            .attr('height', height);
    }

    export function create_sphere(svg, cx, cy, radius) {
        return d3.select(svg)
            .append('circle')
            .attr('cx', cx)
            .attr('cy', cy)
            .attr('r', radius)
            .attr('fill', 'url(#sphere-gradient)')
            .node();
    }

    export function update_sphere_position(sphere, cx, cy) {
        d3.select(sphere)
            .attr('cx', cx)
            .attr('cy', cy);
    }

    export function create_text_element(svg, x, y, char, fill, font_size, skew_x) {
        return d3.select(svg)
            .append('text')
            .attr('x', x)
            .attr('y', y)
            .text(char)
            .attr('fill', fill)
            .attr('font-size', font_size + 'px')
            .attr('font-family', 'Arial, sans-serif')
            .attr('font-weight', 'bold')
            .attr('text-anchor', 'middle')
            .attr('dominant-baseline', 'central')
            .attr('opacity', 1)
            .attr('transform', `skewX(${skew_x})`)
            .node();
    }

    export function update_text_element(element, x, y, font_size, opacity, scale_x, skew_x) {
        // Transform around the text's position, not the SVG origin
        // Order: translate to origin → scale → skew → translate back
        d3.select(element)
            .attr('x', x)
            .attr('y', y)
            .attr('font-size', font_size + 'px')
            .attr('opacity', opacity)
            .attr('transform', `translate(${x}, ${y}) scale(${scale_x}, 1) skewX(${skew_x}) translate(${-x}, ${-y})`);
    }

    export function bring_to_front(element) {
        const node = d3.select(element).node();
        if (node && node.parentNode) {
            node.parentNode.appendChild(node);
        }
    }

    export function send_to_back(element) {
        const node = d3.select(element).node();
        if (node && node.parentNode) {
            node.parentNode.insertBefore(node, node.parentNode.firstChild);
        }
    }

    export function reorder_elements(elements) {
        // elements is an array sorted back-to-front (lowest z first)
        elements.forEach(el => {
            const node = d3.select(el).node();
            if (node && node.parentNode) {
                node.parentNode.appendChild(node);
            }
        });
    }

    export function create_debug_lines(svg, center_x, center_y, width, height) {
        const g = d3.select(svg).append('g').attr('id', 'debug-lines');

        // Vertical line through sphere center
        g.append('line')
            .attr('id', 'sphere-center-line')
            .attr('x1', center_x)
            .attr('y1', 0)
            .attr('x2', center_x)
            .attr('y2', height)
            .attr('stroke', '#ff0000')
            .attr('stroke-width', 2)
            .attr('stroke-dasharray', '5,5')
            .attr('opacity', 0.7);

        // Vertical line for orbit center (same for now)
        g.append('line')
            .attr('id', 'orbit-center-line')
            .attr('x1', center_x)
            .attr('y1', 0)
            .attr('x2', center_x)
            .attr('y2', height)
            .attr('stroke', '#00ff00')
            .attr('stroke-width', 2)
            .attr('stroke-dasharray', '10,10')
            .attr('opacity', 0.7);

        return g.node();
    }

    export function update_debug_info(svg, text) {
        let info = d3.select(svg).select('#debug-info');
        if (info.empty()) {
            info = d3.select(svg).append('text')
                .attr('id', 'debug-info')
                .attr('x', 10)
                .attr('y', 20)
                .attr('fill', '#ffffff')
                .attr('font-size', '12px')
                .attr('font-family', 'monospace');
        }
        info.text(text);
    }

    export function update_debug_lines(svg, center_x, center_y, width, height) {
        const g = d3.select(svg).select('#debug-lines');
        if (!g.empty()) {
            g.select('#sphere-center-line')
                .attr('x1', center_x)
                .attr('x2', center_x)
                .attr('y2', height);

            g.select('#orbit-center-line')
                .attr('x1', center_x)
                .attr('x2', center_x)
                .attr('y2', height);
        }
    }

    export function create_orbit_lines(svg, center_x, center_y, orbit_radius, sphere_offset) {
        const g = d3.select(svg).append('g').attr('id', 'orbit-lines');

        // 0 degrees (right) - Red: x=orbit_radius, z=0
        g.append('line')
            .attr('id', 'orbit-0')
            .attr('x1', center_x + orbit_radius + sphere_offset)
            .attr('y1', center_y - 100)
            .attr('x2', center_x + orbit_radius + sphere_offset)
            .attr('y2', center_y + 100)
            .attr('stroke', '#ff0000')
            .attr('stroke-width', 3)
            .attr('opacity', 0.7);

        // 90 degrees (front) - Green: x=0, z=orbit_radius (closest to viewer)
        g.append('line')
            .attr('id', 'orbit-90')
            .attr('x1', center_x + sphere_offset)
            .attr('y1', center_y - 100)
            .attr('x2', center_x + sphere_offset)
            .attr('y2', center_y + 100)
            .attr('stroke', '#00ff00')
            .attr('stroke-width', 3)
            .attr('opacity', 0.7);

        // 180 degrees (left) - Blue: x=-orbit_radius, z=0
        g.append('line')
            .attr('id', 'orbit-180')
            .attr('x1', center_x - orbit_radius + sphere_offset)
            .attr('y1', center_y - 100)
            .attr('x2', center_x - orbit_radius + sphere_offset)
            .attr('y2', center_y + 100)
            .attr('stroke', '#0000ff')
            .attr('stroke-width', 3)
            .attr('opacity', 0.7);

        // 270 degrees (back) - Yellow: x=0, z=-orbit_radius (furthest from viewer)
        g.append('line')
            .attr('id', 'orbit-270')
            .attr('x1', center_x + sphere_offset)
            .attr('y1', center_y - 100)
            .attr('x2', center_x + sphere_offset)
            .attr('y2', center_y + 100)
            .attr('stroke', '#ffff00')
            .attr('stroke-width', 3)
            .attr('opacity', 0.7);
    }

    export function update_orbit_lines(svg, center_x, center_y, orbit_radius, sphere_offset) {
        const g = d3.select(svg).select('#orbit-lines');
        if (!g.empty()) {
            g.select('#orbit-0')
                .attr('x1', center_x + orbit_radius + sphere_offset)
                .attr('x2', center_x + orbit_radius + sphere_offset);

            g.select('#orbit-90')
                .attr('x1', center_x + sphere_offset)
                .attr('x2', center_x + sphere_offset);

            g.select('#orbit-180')
                .attr('x1', center_x - orbit_radius + sphere_offset)
                .attr('x2', center_x - orbit_radius + sphere_offset);

            g.select('#orbit-270')
                .attr('x1', center_x + sphere_offset)
                .attr('x2', center_x + sphere_offset);
        }
    }
